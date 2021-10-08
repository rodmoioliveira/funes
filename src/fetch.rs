use actix_web::web;
use reqwest::Client;

use crate::error;

pub async fn get(
    client: &web::Data<Client>,
    url: &str,
) -> Result<serde_json::Value, error::FunesError> {
    client
        .get(url)
        .send()
        .await
        .map_err(error::FunesError::Request)?
        .json()
        .await
        .map_err(error::FunesError::Request)
}

pub async fn post(
    client: &web::Data<Client>,
    url: &str,
    payload: &serde_json::Value,
) -> Result<serde_json::Value, error::FunesError> {
    client
        .post(url)
        .json(payload)
        .send()
        .await
        .map_err(error::FunesError::Request)?
        .json()
        .await
        .map_err(error::FunesError::Request)
}
