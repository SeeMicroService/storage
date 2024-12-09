use crate::domain::interfaces::S3Service;
use crate::domain::types::Storage;
use axum::async_trait;
use std::io;
use std::sync::Arc;

pub(crate) struct StorageService {
    storage: Arc<Storage<io::Error>>,
}

impl StorageService {
    pub(crate) fn new(storage: Arc<Storage<io::Error>>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl S3Service for StorageService {
    type Error = io::Error;

    async fn put(&self, filename: &str, content: &[u8]) -> Result<(), Self::Error> {
        self.storage.put(filename, content).await
    }

    async fn remove(&self, filename: &str) -> Result<(), Self::Error> {
        self.storage.remove(filename).await
    }

    async fn get(&self, filename: &str) -> Result<Vec<u8>, Self::Error> {
        self.storage.get(filename).await
    }
}
