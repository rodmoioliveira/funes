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
    UnauthorizedError,
    RequestError(ReqwestError),
    SerdeError(serde_json::Error),
    StdError(std::io::Error),
}

impl std::error::Error for FunesError {}

impl ResponseError for FunesError {
    fn error_response(&self) -> HttpResponse {
        match *self {
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
