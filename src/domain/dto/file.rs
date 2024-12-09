use serde::Deserialize;

#[derive(Deserialize)]
pub struct File {
    pub name: String,
    pub content: String,
}
