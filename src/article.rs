use crate::{prelude::*, response::internal_error_page};

use std::ops::Deref;

use axum::response::Html;
use chroma_syntaxis::SyntaxHighlighter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use once_cell::sync::Lazy;

use crate::{sanitization::sanitize_content, syntax_highlighting::ChromaSyntaxisAdapter};

#[derive(Serialize)]
pub struct Article {
    pub path: String,
    pub title: String,
    pub source: String,
}

pub async fn fetch_article<'e, E>(db: E, path: &str) -> sqlx::Result<Option<Article>>
where
    E: SqliteExecutor<'e>,
{
    sqlx::query_as!(
        Article,
        "SELECT path, title, source FROM articles WHERE path = ?",
        path
    )
    .fetch_optional(db)
    .await
}

static HIGHLIGHTER: Lazy<SyntaxHighlighter> = Lazy::new(SyntaxHighlighter::new);

pub fn render_article_content(source: &str) -> String {
    let highlighter = HIGHLIGHTER.deref();
    let adapter = ChromaSyntaxisAdapter(highlighter);

    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let content = markdown_to_html_with_plugins(source, &options, &plugins);

    sanitize_content(&content).unwrap_or(content)
}

pub async fn article_page(Ext(db): Ext<SqlitePool>, P(article_path): P<String>) -> Response {
    let article_path = article_path.trim_start_matches('/');

    let article = match fetch_article(&db, article_path)
        .await
        .wrap_err("Failed to fetch requested article from database")
    {
        Ok(Some(article)) => article,
        Ok(None) => return "Not Found".into_response(),
        Err(e) => return internal_error_page(e),
    };

    let rendered_content = render_article_content(&article.source);

    Html(
        include_str!("frontend/article.html")
            .replace("[[WIKI:ARTICLE_TITLE]]", &article.title)
            .replace("[[WIKI:ARTICLE_CONTENT]]", &rendered_content),
    )
    .into_response()
}
