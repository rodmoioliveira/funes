use std::{collections::HashMap, time};

use async_std::task;
use rand::Rng;
use serde::Deserialize;

use crate::{api, error, statics};

#[derive(Deserialize, Debug, Clone)]
pub struct Latency {
    pub min: f32,
    pub p50: f32,
    pub p75: f32,
    pub p90: f32,
    pub p95: f32,
    pub p99: f32,
    pub max: f32,
}

impl Latency {
    pub fn default() -> Self {
        Latency {
            min: 0.0,
            p50: 0.0,
            p75: 0.0,
            p90: 0.0,
            p95: 0.0,
            p99: 0.0,
            max: 0.0,
        }
    }
}

pub type Collection = HashMap<String, Latency>;

pub fn key(api: &str) -> Result<&str, error::FunesError> {
    match statics::API_REGEX.find(api) {
        Some(v) => Ok(v.as_str()),
        None => Err(error::FunesError::LatencyCollectionError(api.to_string())),
    }
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn latency(
    api: &String,
    collection: &api::Collection,
) -> Result<time::Duration, error::FunesError> {
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

pub async fn sleep(api: &String, collection: &api::Collection) -> Result<(), error::FunesError> {
    if statics::ENVS.latency_enable {
        let latency = api::latency(api, collection);
        task::sleep(latency?).await;
    }
    Ok(())
}
