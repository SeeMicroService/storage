use crate::domain::types::S3Service;
use std::io;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) service: Arc<S3Service<io::Error>>
}