use crate::model::response::ErrorCode;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[doc = "User model used for authentication"]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[doc = "User response that is generated from @users table"]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl AuthUser {
    #[doc = r#"Retrieve a user from @users by the provided identifier (email or username).
    In any error ErrorCode::INTERNAL001 is returned"#]
    pub async fn find_by_identifier(
        identifier: &str,
        pool: &PgPool,
    ) -> Result<Option<Self>, ErrorCode> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT u.id,
                   u.username,
                   u.email,
                   u.password,
                   u.role
            FROM users u
            WHERE u.username ILIKE $1
               OR u.email ILIKE $1
            "#,
            identifier
        )
        .fetch_optional(pool)
        .await
        .map_err(|_| ErrorCode::INTERNAL001)
    }

    #[doc = "Update the user's last login with the current timestamp."]
    pub async fn update_last_login(&self, pool: &PgPool) -> Result<(), ErrorCode> {
        match sqlx::query!(
            r#"
            UPDATE users u
            SET last_login = NOW()
            WHERE u.id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await
        .map_err(|_| ErrorCode::INTERNAL001)
        {
            Ok(_) => Ok(()),
            Err(c) => Err(c),
        }
    }
}

impl UserResponse {
    #[doc = "Retrieve a user by id, in case of any error ErrorCode::INTERNAL001 is returned"]
    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self, ErrorCode> {
        return sqlx::query_as!(
            Self,
            r#"
            SELECT u.id,
                   u.username,
                   u.email,
                   u.role,
                   u.created_at
            FROM users u
            WHERE u.id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|_| ErrorCode::INTERNAL001);
    }
}
