use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use anyhow::Result;
use log::info;
use reqwest::Url;
use futures_util::StreamExt;

pub async fn export_dataset_to_file(project_id: &str, dataset: &str, token: &str, filepath: &PathBuf) -> Result<()> {
    let endpoint = Url::parse(format!("https://{project_id}.api.sanity.io/v2021-06-07/data/export/{dataset}/").as_str())?;

    info!("Exporting dataset {dataset} from project {project_id} to temporary file {}", filepath.display());

    let mut file = File::options().create(true).read(true).truncate(true).write(true).open(&filepath)?;

    let request = reqwest::Client::new()
        .get(endpoint)
        .header("Authorization", format!("Bearer {token}"));
    let mut bytes_stream = request.send().await?.bytes_stream();

    info!("We have acquired a file stream. Proceeding to write to temporary file.");

    while let Some(chunk) = bytes_stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
    }

    info!("Byte stream successfully written to temporary file.");

    Ok(())
}