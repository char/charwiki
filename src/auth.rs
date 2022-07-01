use std::time::{SystemTime, UNIX_EPOCH};

use axum_auth::AuthBearer;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
}

pub async fn get_user(db: &SqlitePool, auth: Option<AuthBearer>) -> Option<User> {
    let AuthBearer(token) = auth?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    let query = sqlx::query!(
        "SELECT uuid, username
            FROM users, sessions
            WHERE sessions.user_uuid = users.uuid
            AND sessions.token = ?
            AND sessions.expires > ?",
        token,
        now
    );

    let result = query.fetch_one(db).await.ok()?;

    Some(User {
        id: result.uuid.parse().ok()?,
        username: result.username,
    })
}
