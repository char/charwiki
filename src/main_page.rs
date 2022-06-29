use axum::response::Html;
use num_format::{Locale, ToFormattedString};

use crate::{
    article::{render_article_content, Article},
    prelude::*,
    response::internal_error_page,
};

const MAIN_PAGE: &str = include_str!("frontend/main_page.html");

async fn get_main_article(db: &SqlitePool) -> Result<(i32, Option<Article>)> {
    let num_articles = sqlx::query_scalar!("SELECT count(*) FROM articles")
        .fetch_one(db)
        .await?;

    let main_article = sqlx::query_as!(
        Article,
        "SELECT path, title, source FROM articles WHERE path = 'special/main-page'",
    )
    .fetch_optional(db)
    .await?;

    Ok((num_articles, main_article))
}

pub async fn main_page(Ext(db): Ext<SqlitePool>) -> Response {
    let (num_articles, main_article) = match get_main_article(&db).await {
        Ok(r) => r,
        Err(e) => return internal_error_page(e),
    };

    let article_title = main_article
        .as_ref()
        .map(|a| a.title.clone())
        .unwrap_or_else(|| "[not found]".into());

    let rendered_article = main_article
        .as_ref()
        .map(|a| render_article_content(&a.source))
        .unwrap_or_else(|| "The <code>special/main-page</code> article was not found.".into());

    Html(
        MAIN_PAGE
            .replace("[[WIKI:NUM_ARTICLES_RAW]]", &num_articles.to_string())
            .replace(
                "[[WIKI:NUM_ARTICLES_COMMASEP]]",
                &num_articles.to_formatted_string(&Locale::en),
            )
            .replace("[[WIKI:ARTICLE_TITLE]]", &article_title)
            .replace("[[WIKI:ARTICLE_CONTENT]]", &rendered_article),
    )
    .into_response()
}
