use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use std::io;
use std::io::ErrorKind;

pub(crate) fn handle_error(error: io::Error, message: Value) -> (StatusCode, Json<Value>) {
    match error.kind() {
        ErrorKind::NotFound => (StatusCode::NOT_FOUND, Json(message)),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": error.to_string()})),
        ),
    }
}
