use std::{fs, path::Path};

use log::debug;

use crate::{statics, utils};

pub fn check_mocks_dir() -> std::io::Result<()> {
    if !Path::new(&statics::ENVS.mock_dir).exists() {
        fs::create_dir(&statics::ENVS.mock_dir)?;
    }
    Ok(())
}

pub fn read_file(resource: &str) -> Result<String, std::io::Error> {
    let filename = utils::filename(resource);
    let file_content = fs::read_to_string(filename)?;
    Ok(file_content)
}

pub fn write_file(resource: &str, file_content: String) -> Result<(), std::io::Error> {
    let filename = utils::filename(resource);

    debug!("Write filename: {}", filename);
    Ok(fs::write(filename, file_content)?)
}
