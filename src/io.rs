use std::{fs, path::Path};

use crate::{error, format, statics};

pub fn mock_dir() -> std::io::Result<()> {
    if !Path::new(&statics::ENVS.mock_dir).exists() {
        fs::create_dir_all(&statics::ENVS.mock_dir)?;
    }
    Ok(())
}

pub fn read(resource: &str) -> Result<String, error::FunesError> {
    let filename = format::filename(resource);
    fs::read_to_string(filename).map_err(error::FunesError::Std)
}

pub fn write(resource: &str, file_content: String) -> Result<(), error::FunesError> {
    let filename = format::filename(resource);
    fs::write(filename, file_content).map_err(error::FunesError::Std)
}
