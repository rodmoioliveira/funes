use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Deserialize, Serialize)]
pub struct Json<T> {
    pub error: T,
}

impl<T> Json<T> {
    pub fn new(err: T) -> Self {
        Json { error: err }
    }
}

#[derive(Display, From, Debug)]
pub enum FunesError {
    LatencyCollection(String),
    Request(ReqwestError),
    Serde(serde_json::Error),
    Std(std::io::Error),
    Unauthorized,
}

impl std::error::Error for FunesError {}

impl ResponseError for FunesError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            FunesError::LatencyCollection(ref api) => {
                let err_msg = format!("Key {} missing in LATENCY_COLLECTION.", api);
                log::error!("{}", err_msg);
                HttpResponse::InternalServerError().json(Json::new(err_msg))
            }
            FunesError::Request(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            FunesError::Serde(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            FunesError::Std(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            FunesError::Unauthorized => {
                HttpResponse::Unauthorized().json(Json::new("Not allowed to call external apis!"))
            }
        }
    }
}
