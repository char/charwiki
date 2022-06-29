use axum_auth::AuthBearer;

pub async fn is_auth_valid(_auth: Option<AuthBearer>) -> bool {
    // TODO
    true
}
