use std::{collections::HashMap, fs, time};

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;

use crate::{config, latency, statics};

lazy_static! {
    pub static ref API_REGEX: Regex = Regex::new(&ENVS.api_regex).unwrap();

    pub static ref CLIENT: reqwest::Client = Client::builder()
        .user_agent(&statics::ENVS.h_user_agent)
        .timeout(time::Duration::from_millis(2000))
        .build()
        .unwrap();

    #[derive(Debug)]
    pub static ref ENVS: config::Envs = config::Envs::default();

    #[derive(Debug)]
    pub static ref LATENCY_COLLECTION: latency::Collection = match statics::ENVS.latency_enable {
        true => {
            serde_json::from_str(&fs::read_to_string(&statics::ENVS.latency_collection).unwrap())
                .unwrap()
        }
        false => HashMap::new(),
    };
}
