use std::{fs, path::Path};

use crate::{format, statics};

pub fn mock_dir() -> std::io::Result<()> {
    if !Path::new(&statics::ENVS.mock_dir).exists() {
        fs::create_dir_all(&statics::ENVS.mock_dir)?;
    }
    Ok(())
}

pub fn read(resource: &str) -> Result<String, std::io::Error> {
    let filename = format::filename(resource);
    let file_content = fs::read_to_string(filename)?;
    Ok(file_content)
}

pub fn write(resource: &str, file_content: String) -> Result<(), std::io::Error> {
    let filename = format::filename(resource);
    fs::write(filename, file_content)
}
