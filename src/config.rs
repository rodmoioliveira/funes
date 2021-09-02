use std::env;

use serde::Serialize;

use crate::error;

#[derive(Serialize, Clone, Debug)]
pub struct Envs {
    pub allow_externals: bool,
    pub h_server: String,
    pub h_user_agent: String,
    pub localhost: String,
    pub mock_dir: String,
}

impl Envs {
    pub fn allow_externals_calls(&self) -> Result<(), error::FunesError> {
        match self.allow_externals {
            true => Ok(()),
            false => Err(error::FunesError::UnauthorizedError),
        }
    }
}

pub fn envs() -> Envs {
    Envs {
        allow_externals: env::var("RUST_ALLOW_EXTERNALS")
            .unwrap_or("true".to_string())
            .parse::<bool>()
            .unwrap(),
        localhost: env::var("RUST_HOST").unwrap_or("0.0.0.0:8080".to_string()),
        h_user_agent: env::var("RUST_APP").unwrap_or("funes".to_string()),
        h_server: env::var("RUST_APP").unwrap_or("funes".to_string()),
        mock_dir: env::var("RUST_MOCK_DIR").unwrap_or("./mocks".to_string()),
    }
}
