use axum::Json;
use color_eyre::Report;
use hyper::StatusCode;
use serde_json::json;

use crate::prelude::*;

fn internal_error_message(_err: Report) -> String {
    // TODO: Report the error backtrace if in dev mode
    "Internal server error :(".into()
}

fn not_found_message(what: &str) -> String {
    format!("The requested {what} was not found :(")
}

pub fn internal_error_json(err: Report) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": internal_error_message(err) })),
    )
        .into_response()
}

pub fn internal_error_page(err: Report) -> Response {
    let message = internal_error_message(err);
    (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
}

pub fn not_found_json(what: &str) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": not_found_message(what) })),
    )
        .into_response()
}

pub fn unauthorized_json() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Invalid authentication." })),
    )
        .into_response()
}
