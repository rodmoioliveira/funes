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
    LatencyCollectionError(String),
    RequestError(ReqwestError),
    SerdeError(serde_json::Error),
    StdError(std::io::Error),
    UnauthorizedError,
}

impl std::error::Error for FunesError {}

impl ResponseError for FunesError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            FunesError::LatencyCollectionError(ref api) => {
                let err_msg = format!("Key {} missing in LATENCY_COLLECTION.", api);
                log::error!("{}", err_msg);
                HttpResponse::InternalServerError().json(Json::new(err_msg))
            }
            FunesError::RequestError(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            FunesError::SerdeError(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            FunesError::StdError(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            FunesError::UnauthorizedError => {
                HttpResponse::Unauthorized().json(Json::new("Not allowed to call external apis!"))
            }
        }
    }
}
