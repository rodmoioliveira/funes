use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    time,
};

use async_std::task;
use rand::Rng;
use serde::Deserialize;

use crate::{config, error, statics};

static ASYNC_TASK_SLEEP_MODIFIER: u64 = 87;

#[derive(Deserialize, Debug, Clone)]
pub struct Distribution {
    pub min: u64,
    pub p50: u64,
    pub p75: u64,
    pub p90: u64,
    pub p95: u64,
    pub p99: u64,
    pub max: u64,
}

pub type Collection = HashMap<String, Distribution>;

pub fn key(api: &str) -> Result<&str, error::FunesError> {
    match statics::API_REGEX.find(api) {
        Some(v) => Ok(v.as_str()),
        None => Err(error::FunesError::LatencyCollection(api.to_string())),
    }
}

fn latency(api: &str, collection: &Collection) -> Result<time::Duration, error::FunesError> {
    let key = key(api)?;
    let latency = collection.get(key).unwrap();
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..=100);

    let api_res_time = match random {
        0..=50 => latency.p50,
        51..=75 => latency.p75,
        76..=90 => latency.p90,
        91..=95 => latency.p95,
        96..=99 => latency.p99,
        100 => latency.max,
        _ => latency.min,
    };

    Ok(time::Duration::from_millis(
        api_res_time / ASYNC_TASK_SLEEP_MODIFIER,
    ))
}

pub async fn sleep(api: &str, collection: &Collection) -> Result<(), error::FunesError> {
    if statics::ENVS.latency_enable {
        let latency = latency(api, collection);
        task::sleep(latency?).await;
    }
    Ok(())
}

pub fn validate() {
    if statics::ENVS.latency_enable {
        let keys = statics::LATENCY_COLLECTION
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        let regex_map: Vec<_> = keys
            .iter()
            .map(|v| key(v).unwrap_or("None").to_owned())
            .collect();

        if keys != regex_map {
            let keys_set: HashSet<String> = HashSet::from_iter(keys.iter().cloned());
            let regex_map_set: HashSet<String> = regex_map.iter().clone().cloned().collect();
            let diff: HashSet<_> = keys_set.difference(&regex_map_set).collect();

            log::error!(
                "\n\n{}: \"{}\"\nDoesn`t fully match all keys from {}: {:#?}\n\nUnmatched keys \
                 are:\n {:#?}\n\n",
                config::FUNES_API_REGEX,
                statics::ENVS.api_regex,
                config::FUNES_LATENCY_COLLECTION,
                keys_set,
                diff,
            );
            panic!("Unmatched keys in {}", config::FUNES_LATENCY_COLLECTION);
        }
    }
}
