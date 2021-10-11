use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    time,
};

use async_std::task;
use rand::Rng;
use serde::Deserialize;

use crate::{error, statics};

#[derive(Deserialize, Debug, Clone)]
pub struct Distribution {
    pub min: f32,
    pub p50: f32,
    pub p75: f32,
    pub p90: f32,
    pub p95: f32,
    pub p99: f32,
    pub max: f32,
}

pub type Collection = HashMap<String, Distribution>;

pub fn key(api: &str) -> Result<&str, error::FunesError> {
    match statics::API_REGEX.find(api) {
        Some(v) => Ok(v.as_str()),
        None => Err(error::FunesError::LatencyCollection(api.to_string())),
    }
}

pub fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn latency(api: &str, collection: &Collection) -> Result<time::Duration, error::FunesError> {
    let key = key(api)?;
    let latency = collection.get(key).unwrap();
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..=100);

    let api_res_time = match random {
        0..=50 => map_range((0.0, 50.0), (latency.min, latency.p50), random as f32),
        51..=75 => map_range((51.0, 75.0), (latency.p50, latency.p75), random as f32),
        76..=90 => map_range((76.0, 90.0), (latency.p75, latency.p90), random as f32),
        91..=95 => map_range((91.0, 95.0), (latency.p90, latency.p95), random as f32),
        96..=99 => map_range((96.0, 99.0), (latency.p95, latency.p99), random as f32),
        100 => latency.max,
        _ => 0.0,
    };

    Ok(time::Duration::from_secs_f32(api_res_time))
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
                "FUNES_LATENCY_COLLECTION keys:\n {:#?}\n\nDiffer from \
                 regex_map:\n{:#?}\n\nBecause FUNES_API_REGEX: {} doesn't match all keys.\nThe \
                 offenders are the following:\n\n{:?}\n",
                keys_set,
                regex_map_set,
                statics::ENVS.api_regex,
                diff,
            );
            panic!();
        }
    }
}
