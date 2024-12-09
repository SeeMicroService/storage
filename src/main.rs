use crate::application::{State, StorageService};
use crate::domain::types::Error;
use crate::infrastructure::Minio;
use axum::routing::{delete, get, put};
use axum::Router;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

mod application;
mod domain;
mod handlers;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if dotenvy::dotenv().is_err() {
        println!("Failed to load .env, trying to use env variables");
    }
    tracing_subscriber::fmt::init();

    let endpoint = std::env::var("STORAGE_ENDPOINT").expect("STORAGE_ENDPOINT must be set");
    let hostaddr = std::env::var("HOSTADDR").expect("HOSTADDR must be set");

    let access_key = std::env::var("ACCESS_KEY").expect("ACCESS_KEY must be set");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let bucket_name = std::env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");

    let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)?;
    let region = Region::Custom {
        region: String::default(),
        endpoint,
    };
    let mut bucket = Bucket::new(&bucket_name, region, credentials)?;
    bucket.set_path_style();

    let storage = Arc::new(Minio::new(bucket));
    let service = Arc::new(StorageService::new(storage));
    let state = State { service };

    let app = Router::new()
        .route("/delete", delete(handlers::delete))
        .route("/get_file", get(handlers::get_file))
        .route("/put_file", put(handlers::put_file))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&hostaddr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
