use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Mock {
    pub body: serde_json::Value,
    pub headers: String,
    pub method: String,
    pub payload: serde_json::Value,
    pub resource: String,
    pub status: u16,
    pub version: String,
}
