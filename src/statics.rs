use std::time;

use lazy_static::lazy_static;
use reqwest::Client;

use crate::{models, statics, utils};

lazy_static! {
    pub static ref ENVS: models::Envs = utils::envs();
    pub static ref CLIENT: reqwest::Client = Client::builder()
        .user_agent(statics::ENVS.h_user_agent.clone())
        .timeout(time::Duration::from_millis(2000))
        .build()
        .unwrap();
}
