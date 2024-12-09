use axum::async_trait;

#[async_trait]
pub(crate) trait S3Service {
    type Error;
    
    async fn put(&self, filename: &str, content: &[u8]) -> Result<(), Self::Error>;
    async fn remove(&self, filename: &str) -> Result<(), Self::Error>;
    async fn get(&self, filename: &str) -> Result<Vec<u8>, Self::Error>;
}