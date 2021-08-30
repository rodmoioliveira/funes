use std::time;

use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    pub static ref CLIENT: reqwest::Client = Client::builder()
        .timeout(time::Duration::from_millis(2000))
        .build()
        .unwrap();
}

pub static MOCK_DIR: &str = "./mocks";
