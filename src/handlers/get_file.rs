use crate::application;
use crate::domain::dto::Path;
use crate::handlers::error::handle_error;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;

pub(crate) async fn get_file(
    State(state): State<application::State>,
    Json(path): Json<Path>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let result = state.service.get(&path.path).await;
    match result {
        Ok(file) => {
            let file = Body::from(file);
            Ok((StatusCode::OK, file))
        }
        Err(error) => Err(handle_error(error, Value::default())),
    }
}
