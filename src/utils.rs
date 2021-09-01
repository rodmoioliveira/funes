use std::{
    collections::hash_map::DefaultHasher,
    env, fs,
    hash::{Hash, Hasher},
    path::Path,
};

use log::debug;

use crate::{models, statics};

pub fn check_mocks_dir() -> std::io::Result<()> {
    if !Path::new(&statics::ENVS.mock_dir).exists() {
        fs::create_dir(&statics::ENVS.mock_dir)?;
    }
    Ok(())
}

pub fn envs() -> models::Envs {
    models::Envs {
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

pub fn filename(resource: &str) -> String {
    format!(
        "{}/{}.json",
        &statics::ENVS.mock_dir,
        calculate_hash(&resource)
    )
}

pub fn hash_separator(qs: &str) -> &str {
    match qs == "" {
        true => "",
        false => "-",
    }
}

pub fn qs_separator(qs: &str) -> &str {
    match qs == "" {
        true => "",
        false => "?",
    }
}

pub fn read_file(resource: &str) -> Result<String, std::io::Error> {
    let filename = filename(resource);
    let file_content = fs::read_to_string(filename)?;
    Ok(file_content)
}

pub fn resource(api: &str, qs: &str, hash: &str) -> String {
    let resource = format!(
        "{}{}{}{}{}",
        api,
        qs_separator(qs),
        qs,
        hash_separator(hash),
        hash
    );
    resource
}

pub fn url(api: &str, qs: &str) -> String {
    format!("http://{}{}{}", &api, qs_separator(qs), qs)
}

pub fn write_file(resource: &str, file_content: String) -> Result<(), std::io::Error> {
    let filename = filename(resource);

    debug!("Write filename: {}", filename);
    Ok(fs::write(filename, file_content)?)
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
