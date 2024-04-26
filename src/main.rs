mod sanity;
mod s3;

use log::info;
use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting backup from Sanity to an AWS S3 bucket.");

    let sanity_dataset = get_from_env("SANITY_DATASET");
    let sanity_project_id = get_from_env("SANITY_PROJECT_ID");
    let sanity_token = get_from_env("SANITY_TOKEN");

    // We create this here and pass around as it will be deleted once it is out of scope.
    let tmpdir = tempfile::tempdir()?;
    let filepath = tmpdir.path().join("export.ndjson");

    sanity::export_dataset_to_file(sanity_project_id.as_str(), sanity_dataset.as_str(), sanity_token.as_str(), &filepath).await?;

    info!("Backup to temporary file succeeded.");

    info!("Initialising AWS S3 client.");
    let s3_client = configure_aws_s3_client().await;

    let bucket = get_from_env("S3_BUCKET");

    s3::upload_backup_to_s3(&s3_client, bucket.as_str(), &filepath, &sanity_dataset).await?;

    Ok(())
}

fn get_from_env(key: &str) -> String {
    std::env::var(key).expect(&format!("Environment variable {key} must be set."))
}

async fn configure_aws_s3_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-north-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load().await;
    Client::new(&config)
}

