use actix_web::web;
use reqwest::Client;

use crate::error;

pub async fn get(
    client: &web::Data<Client>,
    url: &String,
) -> Result<serde_json::Value, error::FunesError> {
    Ok(client
        .get(url)
        .send()
        .await
        .map_err(error::FunesError::RequestError)?
        .json()
        .await
        .map_err(error::FunesError::RequestError)?)
}

pub async fn post(
    client: &web::Data<Client>,
    url: &String,
    payload: &serde_json::Value,
) -> Result<serde_json::Value, error::FunesError> {
    Ok(client
        .post(url)
        .json(payload)
        .send()
        .await
        .map_err(error::FunesError::RequestError)?
        .json()
        .await
        .map_err(error::FunesError::RequestError)?)
}
