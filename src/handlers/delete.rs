use crate::application;
use crate::domain::dto::Path;
use crate::handlers::error::handle_error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub(crate) async fn delete(
    State(state): State<application::State>,
    Json(path): Json<Path>,
) -> (StatusCode, Json<Value>) {
    let result = state.service.remove(&path.path).await;
    match result {
        Ok(_) => (StatusCode::NO_CONTENT, Json(json!({}))),
        Err(error) => handle_error(error, json!({"error": "No such path"})),
    }
}
