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
    method: reqwest::Method,
    resource: &str,
    payload: Option<&serde_json::Value>,
    version: reqwest::Version,
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    body: &serde_json::Value,
) -> String {
    format!(
        "Resource: {resource}{file_sep}Version: {version:?}{file_sep}Method: \
         {method}{file_sep}Status: {status}{file_sep}Payload: {payload:}{file_sep}Response \
         Headers: {headers:?}{file_sep}Body: {content}",
        resource = resource,
        version = version,
        method = method.as_str(),
        status = status.as_str(),
        payload = serde_json::to_string(payload.unwrap_or(&serde_json::json!({}))).unwrap(),
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
