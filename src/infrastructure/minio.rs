use crate::domain::interfaces::Storage;
use axum::async_trait;
use s3::error::S3Error;
use s3::Bucket;
use std::io;

pub struct Minio {
    bucket: Box<Bucket>,
}

impl Minio {
    pub fn new(bucket: Box<Bucket>) -> Minio {
        Self { bucket }
    }

    fn map_error(error: S3Error) -> io::Error {
        match error {
            S3Error::Io(io_error) => io_error,
            _ => {
                eprint!("{}", error);
                io::Error::new(io::ErrorKind::Other, "An unknown error occurred.")
            },
        }
    }
}

#[async_trait]
impl Storage for Minio {
    type Error = io::Error;

    async fn put(&self, filename: &str, content: &[u8]) -> Result<(), Self::Error> { 
        let result = self.bucket.put_object(filename, &content).await;
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(Self::map_error(error))
        }
    }

    async fn remove(&self, filename: &str) -> Result<(), Self::Error> {
        let result = self.bucket.delete_object(filename).await;
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(Self::map_error(error)),
        }
    }

    async fn get(&self, filename: &str) -> Result<Vec<u8>, Self::Error> {
        let result = self.bucket.get_object(filename).await;
        match result {
            Ok(content) => Ok(content.to_vec()),
            Err(error) => Err(Self::map_error(error)),
        }
    }
}
