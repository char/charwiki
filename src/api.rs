use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_auth::AuthBearer;
use serde_json::json;

use crate::{
    article::Article,
    auth::is_auth_valid,
    prelude::*,
    response::{internal_error_json, not_found_json, unauthorized_json},
};

pub async fn get_article(Ext(db): Ext<SqlitePool>, P(article_path): P<String>) -> Response {
    let query = sqlx::query_as!(
        Article,
        "SELECT path, title, source FROM articles WHERE path = ?",
        article_path
    );

    let article = match query
        .fetch_optional(&db)
        .await
        .wrap_err("Failed to query database for article")
    {
        Ok(Some(article)) => article,
        Ok(None) => return not_found_json("article"),
        Err(err) => return internal_error_json(err),
    };

    Json(article).into_response()
}

#[derive(Deserialize)]
pub struct ArticleRequestBody {
    pub title: String,
    pub source: String,
}

pub async fn rewrite_article(
    Ext(db): Ext<SqlitePool>,
    P(article_path): P<String>,
    Json(body): Json<ArticleRequestBody>,
    auth: Option<AuthBearer>,
) -> Response {
    if !is_auth_valid(auth).await {
        return unauthorized_json();
    }

    let query = sqlx::query!(
        "UPDATE articles SET title = ?, source = ? WHERE path = ?",
        body.title,
        body.source,
        article_path
    );

    let result = match query
        .execute(&db)
        .await
        .wrap_err("Failed to update article in database")
    {
        Ok(r) => r,
        Err(err) => return internal_error_json(err),
    };

    if result.rows_affected() == 0 {
        return not_found_json("article");
    }

    Json(json!({"success": true})).into_response()
}

pub async fn create_article(
    Ext(db): Ext<SqlitePool>,
    P(article_path): P<String>,
    Json(body): Json<ArticleRequestBody>,
    auth: Option<AuthBearer>,
) -> Response {
    if !is_auth_valid(auth).await {
        return unauthorized_json();
    }

    let query = sqlx::query!(
        "INSERT OR IGNORE INTO articles (path, title, source) VALUES (?, ?, ?)",
        article_path,
        body.title,
        body.source
    );

    let _ = match query
        .execute(&db)
        .await
        .wrap_err("Failed to create article")
    {
        Ok(r) => r,
        Err(err) => return internal_error_json(err),
    };

    Json(json!({"success": true})).into_response()
}
