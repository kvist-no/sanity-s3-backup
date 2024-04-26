use std::path::{PathBuf};
use aws_sdk_s3::Client;
use anyhow::Result;
use aws_sdk_s3::primitives::{ByteStream};
use log::info;
use chrono::Utc;

pub async fn upload_backup_to_s3(s3_client: &Client, bucket_name: &str, filepath: &PathBuf, dataset: &str) -> Result<()> {
    info!("Beginning upload of backup file to S3 bucket {bucket_name}.");

    let body = ByteStream::from_path(filepath).await?;

    s3_client
        .put_object()
        .bucket(bucket_name)
        .key(format!("{}-{}.ndjson", dataset, Utc::now().to_string()))
        .body(body)
        .send()
        .await?;

    info!("File successfully uploaded to S3 bucket");

    Ok(())
}