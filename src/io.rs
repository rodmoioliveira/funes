use std::{fs, path::Path};

use crate::{format, mock, statics};

pub fn mock_dir() -> std::io::Result<()> {
    if !Path::new(&statics::ENVS.mock_dir).exists() {
        fs::create_dir_all(&statics::ENVS.mock_dir)?;
    }
    Ok(())
}

pub fn read(resource: &str) -> Result<mock::Mock, std::io::Error> {
    println!("READ");
    let filename = format::filename(resource);
    let file_content: mock::Mock = serde_json::from_str(&fs::read_to_string(&filename)?).unwrap();
    Ok(file_content)
}

pub async fn write(
    resource: &str,
    res: reqwest::Response,
    method: reqwest::Method,
    payload: Option<&serde_json::Value>,
) -> Result<(reqwest::StatusCode, serde_json::Value), std::io::Error> {
    println!("WRITE");
    let filename = format::filename(resource);
    let version = res.version();
    let headers = res.headers().clone();
    let status = res.status();
    let body: serde_json::Value = res.json().await.unwrap_or_else(|_| serde_json::json!({}));

    let file_content = mock::Mock {
        resource: resource.to_string(),
        method: method.as_str().to_string(),
        version: format!("{:?}", version),
        headers: format!("{:?}", headers),
        status: reqwest::StatusCode::as_u16(&status),
        payload: payload.unwrap_or(&serde_json::json!({})).clone(),
        body: body.clone(),
    };
    fs::write(filename, serde_json::to_string(&file_content).unwrap())?;

    Ok((status, body))
}
