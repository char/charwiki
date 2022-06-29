mod prelude;

mod api;
mod article;
mod auth;
mod main_page;
mod response;
mod sanitization;
mod syntax_highlighting;

use crate::prelude::*;

use std::net::SocketAddr;

use axum::{
    body::{boxed as box_body, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
    routing::get,
    Router, Server,
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

async fn connect_db() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL").wrap_err("DATABASE_URL was not defined!")?;

    SqlitePool::connect(&database_url)
        .await
        .wrap_err("Failed to connect to the SQLite database")
}

async fn listen(app: Router) -> Result<()> {
    let bind_addr = std::env::var("BIND_ADDR")
        .ok()
        .map(|s| {
            s.parse::<SocketAddr>()
                .wrap_err_with(|| format!("Invalid socket bind address {s}"))
        })
        .transpose()?
        .unwrap_or_else(|| ([127, 0, 0, 1], 3000).into());

    println!("Listening at: http://{bind_addr}/ ...");
    Server::bind(&bind_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn static_handler(u: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(u).body(Body::empty()).unwrap();

    match ServeDir::new("./static").oneshot(req).await {
        Ok(res) => Ok(res.map(box_body)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;
    let db = connect_db().await?;

    let app = Router::new()
        .route(
            "/api/article/*article_path",
            get(api::get_article)
                .patch(api::rewrite_article)
                .put(api::create_article),
        )
        .route("/", get(main_page::main_page))
        .route("/-/*article_path", get(article::article_page))
        .nest("/static", get(static_handler))
        .layer(Ext(db));

    listen(app).await
}
