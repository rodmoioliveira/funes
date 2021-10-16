use std::{env, fs};

use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;

use crate::{config, latency};

lazy_static! {
    // For now, this is just a magic number to adjust the sleep time for
    // async_std::task::sleep.
    pub static ref ASYNC_TASK_SLEEP_MODIFIER: u64 = env::var("ASYNC_TASK_SLEEP_MODIFIER")
        .unwrap_or_else(|_| "68".to_string())
        .parse::<u64>()
        .unwrap();

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
