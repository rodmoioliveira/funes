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
pub enum MyError {
    UnauthorizedError,
    RequestError(ReqwestError),
    SerdeError(serde_json::Error),
    StdError(std::io::Error),
}

impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::RequestError(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            MyError::SerdeError(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            MyError::StdError(ref err) => {
                HttpResponse::InternalServerError().json(Json::new(err.to_string()))
            }
            MyError::UnauthorizedError => {
                HttpResponse::Unauthorized().json(Json::new("Not allowed to call external apis!"))
            }
        }
    }
}
