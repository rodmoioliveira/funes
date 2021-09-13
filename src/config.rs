use std::env;

use serde::Serialize;

use crate::error;

#[derive(Serialize, Clone, Debug)]
pub struct Envs {
    pub allow_externals: bool,
    pub api_regex: String,
    pub h_server: String,
    pub h_user_agent: String,
    pub latency_collection: String,
    pub latency_enable: bool,
    pub localhost: String,
    pub log: String,
    pub mock_dir: String,
}

impl Envs {
    pub fn default() -> Self {
        let latency_collection =
            env::var("FUNES_LATENCY_COLLECTION").unwrap_or_else(|_| "".to_string());
        let latency_enable = !latency_collection.is_empty();

        Envs {
            allow_externals: env::var("FUNES_ALLOW_EXTERNALS")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>()
                .unwrap(),
            api_regex: env::var("FUNES_API_REGEX").unwrap_or_else(|_| r".+".to_string()),
            h_server: env::var("FUNES_APP").unwrap_or_else(|_| "funes".to_string()),
            h_user_agent: env::var("FUNES_APP").unwrap_or_else(|_| "funes".to_string()),
            latency_collection,
            latency_enable,
            localhost: env::var("FUNES_HOST").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            log: env::var("FUNES_LOG").unwrap_or_else(|_| "funes,actix_web=info".to_string()),
            mock_dir: env::var("FUNES_MOCK_DIR").unwrap_or(format!(
                "{}/.funes",
                dirs::home_dir().unwrap().to_str().unwrap().to_string()
            )),
        }
    }

    pub fn allow_externals_calls(&self) -> Result<(), error::FunesError> {
        match self.allow_externals {
            true => Ok(()),
            false => Err(error::FunesError::Unauthorized),
        }
    }
}
