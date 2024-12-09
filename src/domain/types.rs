use crate::domain::interfaces;

pub(crate) type Error = Box<dyn std::error::Error + Sync + Send>;

pub(crate) type Storage<E> = dyn interfaces::Storage<Error = E> + Sync + Send;
pub(crate) type S3Service<E> = dyn interfaces::S3Service<Error = E> + Sync + Send;
