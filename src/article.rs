use std::ops::Deref;

use chroma_syntaxis::SyntaxHighlighter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::{sanitization::sanitize_content, syntax_highlighting::ChromaSyntaxisAdapter};

#[derive(Serialize)]
pub struct Article {
    pub path: String,
    pub title: String,
    pub source: String,
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
