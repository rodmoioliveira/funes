use crate::{statics, utils};

pub fn resource(api: &str, qs: &str, hash: &str) -> String {
    let resource = format!(
        "{}{}{}{}{}",
        api,
        utils::sep_qs(qs),
        qs,
        utils::sep_hash(hash),
        hash
    );
    resource
}

pub fn content(
    resource: &str,
    version: reqwest::Version,
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    body: &serde_json::Value,
) -> String {
    format!(
        "{resource}{file_sep}{version:?}{file_sep}{status}{file_sep}{headers:?}{file_sep}{content}",
        resource = resource,
        version = version,
        status = status.as_str(),
        headers = headers,
        content = serde_json::to_string(body).unwrap_or_else(|_| "".to_string()),
        file_sep = *statics::FILE_CONTENT_SEP,
    )
}

pub fn filename(resource: &str) -> String {
    format!("{}/{}", &statics::ENVS.mock_dir, utils::hash(&resource))
}

pub fn url(api: &str, qs: &str) -> String {
    format!("http://{}{}{}", &api, utils::sep_qs(qs), qs)
}
