use std::time;

use lazy_static::lazy_static;
use reqwest::Client;

use crate::{config, statics};

lazy_static! {
    pub static ref ENVS: config::Envs = config::envs();
    pub static ref CLIENT: reqwest::Client = Client::builder()
        .user_agent(&statics::ENVS.h_user_agent)
        .timeout(time::Duration::from_millis(2000))
        .build()
        .unwrap();
}
