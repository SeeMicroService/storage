use crate::application;
use crate::domain::dto::File;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub(crate) async fn put_file(
    State(state): State<application::State>,
    Json(file): Json<File>,
) -> (StatusCode, Json<Value>) {
    let result = state.service.put(&file.name, file.content.as_bytes()).await;
    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({}))),
        Err(error) => handle_error(error, Value::default()),
    }
}
