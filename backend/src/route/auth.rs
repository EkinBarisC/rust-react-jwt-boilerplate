use crate::domain::user::UserResponse;
use crate::middleware::auth::AuthDetails;
use crate::model::request::auth::LoginRequest;
use crate::model::request::registration::RegistrationRequest;
use crate::model::response::{ErrorCode, ErrorResponse};
use crate::server::AppState;
use crate::service::authentication::{login_user, refresh_access_token};
use crate::service::registration::register_new_user;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpRequest, HttpResponse};
use serde_json::json;

#[doc = r#"API Resource: /auth/register [POST]

Register a new user to the application providing the required details.

If the operation is successfull, 200 OK is returned with the created user's details.

Errors:

ErrorCode::REG001 / 409 Conflict - The username already exists.
ErrorCode::REG002 / 409 Conflict - The email already exists.
ErrorCode::REG003 / 500 Internal Server Error - Any other error case.
"#]
#[post("/auth/register")]
pub async fn register_user_handler(
    body: Json<RegistrationRequest>,
    data: Data<AppState>,
) -> HttpResponse {
    let response: Result<UserResponse, ErrorCode> =
        register_new_user(body.into_inner(), &data.db).await;

    match response {
        Ok(user) => HttpResponse::Ok().json(json!(user)),
        Err(c) => ErrorResponse::build(c),
    }
}

#[doc = r#"API Resource: /auth/login [POST]

Login the user that matches the provided credentials to the application.

If successfull, a cookie is set with the JWT token for the user. 200 Ok is returned with the token value as well.

ErrorCode::AUTH001 / 404 Not Found - The user was not found in the database.
ErrorCode::AUTH002 / 400 Bad Request - The user's password is wrong.
ErrorCode::INTERNAL001 / 500 Bad Request - Any other error.
"#]
#[post("/auth/login")]
pub async fn login_handler(body: Json<LoginRequest>, data: Data<AppState>) -> HttpResponse {
    let login_result: Result<(String, String), ErrorCode> =
        login_user(body.into_inner(), &data.settings, &data.db).await;

    match login_result {
        Ok((access_token, refresh_token)) => HttpResponse::Ok()
            .cookie(
                Cookie::build(&data.settings.jwt.cookie_name, access_token.to_owned())
                    .path("/")
                    .domain(&data.settings.jwt.cookie_domain.clone())
                    .http_only(true)
                    .secure(false)
                    .same_site(SameSite::Lax)
                    .finish(),
            )
            .cookie(
                Cookie::build("refresh_token", refresh_token.to_owned())
                    .path("/")
                    .domain(&data.settings.jwt.cookie_domain.clone())
                    .http_only(true)
                    .secure(false)
                    .same_site(SameSite::Lax)
                    .finish(),
            )
            .json(json!({"token": access_token})),
        Err(code) => ErrorResponse::build(code),
    }
}
#[post("/auth/refresh")]
pub async fn refresh_token_handler(req: HttpRequest, data: Data<AppState>) -> HttpResponse {
    let refresh_token = match req.cookie("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return HttpResponse::Unauthorized().json(json!({"error": "Refresh token not found"}))
        }
    };

    let new_access_token = refresh_access_token(refresh_token, &data.settings).await;

    match new_access_token {
        Ok(token) => HttpResponse::Ok()
            .cookie(
                Cookie::build(&data.settings.jwt.cookie_name, token.to_owned())
                    .path("/")
                    .domain(&data.settings.jwt.cookie_domain.clone())
                    .http_only(true)
                    .secure(false)
                    .same_site(SameSite::Lax)
                    .finish(),
            )
            .json(json!({"token": token})),
        Err(code) => ErrorResponse::build(code),
    }
}
#[doc = r#"API Resource: /auth/logout \[POST\]

Logout the provided user by deleting the token cookie.

Returns:

204 / No Content if the operation is successfull.
"#]
#[post("/auth/logout")]
pub async fn logout_handler(_auth: AuthDetails, data: Data<AppState>) -> HttpResponse {
    HttpResponse::NoContent()
        .cookie(
            Cookie::build(&data.settings.jwt.cookie_name, "")
                .path("/")
                .domain(&data.settings.jwt.cookie_domain.clone())
                .http_only(true)
                .secure(false)
                .finish(),
        )
        .cookie(
            Cookie::build("refresh_token", "")
                .path("/")
                .domain(&data.settings.jwt.cookie_domain.clone())
                .http_only(true)
                .secure(false)
                .finish(),
        )
        .finish()
}

#[doc = r#"API Resource: /auth/profile \[GET\]

Retrieve the profile of the logged in user.

Returns:

200 / Ok - if the user is found
"#]
#[get("/auth/profile")]
pub async fn profile_handler(auth: AuthDetails, data: Data<AppState>) -> HttpResponse {
    match UserResponse::find_by_id(auth.user_id, &data.db).await {
        Ok(u) => HttpResponse::Ok().json(json!(u)),
        Err(e) => ErrorResponse::build(e),
    }
}
