use std::fs;

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;

use crate::{config, latency};

lazy_static! {
    pub static ref API_REGEX: Regex = Regex::new(&LATENCY_COLLECTION.regex).unwrap();

    pub static ref CLIENT: reqwest::Client = Client::builder()
        .user_agent(&ENVS.h_user_agent)
        .build()
        .unwrap();

    #[derive(Debug)]
    pub static ref ENVS: config::Envs = config::Envs::default();

    #[derive(Debug)]
    pub static ref LATENCY_COLLECTION: latency::Collection = match ENVS.latency_enable {
        true => {
            serde_json::from_str(&fs::read_to_string(&ENVS.latency_collection).unwrap())
                .unwrap()
        }
        false => latency::Collection::default()
    };
}
