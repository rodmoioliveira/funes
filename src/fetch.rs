use actix_web::web;
use log::debug;
use reqwest::Client;

use crate::error;

pub async fn get(
    client: &web::Data<Client>,
    url: &String,
) -> Result<serde_json::Value, error::MyError> {
    debug!("External get to: {}", url);
    Ok(client
        .get(url)
        .send()
        .await
        .map_err(error::MyError::ReqwestError)?
        .json()
        .await
        .map_err(error::MyError::ReqwestError)?)
}

pub async fn post(
    client: &web::Data<Client>,
    url: &String,
    payload: &serde_json::Value,
) -> Result<serde_json::Value, error::MyError> {
    debug!("External post to: {}", url);
    Ok(client
        .post(url)
        .json(payload)
        .send()
        .await
        .map_err(error::MyError::ReqwestError)?
        .json()
        .await
        .map_err(error::MyError::ReqwestError)?)
}
