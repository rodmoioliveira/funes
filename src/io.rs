use std::{fs, path::Path};

use crate::{format, statics};

pub fn mock_dir() -> std::io::Result<()> {
    if !Path::new(&statics::ENVS.mock_dir).exists() {
        fs::create_dir_all(&statics::ENVS.mock_dir)?;
    }
    Ok(())
}

pub fn read(resource: &str) -> Result<(reqwest::StatusCode, serde_json::Value), std::io::Error> {
    let filename = format::filename(resource);
    let file_content = fs::read_to_string(&filename)?;
    let content: Vec<&str> = file_content.split(&*statics::FILE_CONTENT_SEP).collect();
    let status = reqwest::StatusCode::from_u16(content[2].parse::<u16>().unwrap()).unwrap();
    let body: serde_json::Value = serde_json::from_str(content[4])?;

    Ok((status, body))
}

pub async fn write(
    resource: &str,
    res: reqwest::Response,
) -> Result<(reqwest::StatusCode, serde_json::Value), std::io::Error> {
    let filename = format::filename(resource);
    let version = res.version();
    let headers = res.headers().clone();
    let status = res.status();
    let body: serde_json::Value = res.json().await.unwrap_or_else(|_| serde_json::json!({}));

    let file_content = format::content(resource, version, status, headers, &body);
    fs::write(filename, file_content)?;

    Ok((status, body))
}
