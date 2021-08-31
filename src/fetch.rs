use actix_web::{http, web};
use log::debug;
use reqwest::Client;

use crate::{error, utils};

pub async fn get(
    client: &web::Data<Client>,
    url: &String,
    headers: &http::HeaderMap,
) -> Result<serde_json::Value, error::MyError> {
    debug!("External get to: {}", url);
    Ok(client
        .get(url)
        .headers(utils::convert_headers(headers))
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
    headers: &http::HeaderMap,
) -> Result<serde_json::Value, error::MyError> {
    debug!("External post to: {}", url);
    Ok(client
        .post(url)
        .headers(utils::convert_headers(headers))
        .json(payload)
        .send()
        .await
        .map_err(error::MyError::ReqwestError)?
        .json()
        .await
        .map_err(error::MyError::ReqwestError)?)
}
