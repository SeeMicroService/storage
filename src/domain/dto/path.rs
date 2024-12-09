use serde::Deserialize;

#[derive(Deserialize)]
pub struct Path {
    pub path: String,
}