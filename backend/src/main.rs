use std::net::TcpListener;

use backend::{config::app_settings::get_configuration, server::run};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = get_configuration().expect("Failed to read configuration");
    let pg_pool = PgPoolOptions::new().connect_lazy_with(settings.database.get_options());
    let listener =
        TcpListener::bind(settings.application.get_addr()).expect("Failed to bind address");

    println!(
        "Server running at http://{}",
        listener.local_addr().unwrap()
    );

    run(listener, settings, pg_pool).await?.await
}
