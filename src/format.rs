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

pub fn filename(resource: &str) -> String {
    format!(
        "{}/{}.json",
        &statics::ENVS.mock_dir,
        utils::hash(&resource)
    )
}

pub fn url(api: &str, qs: &str) -> String {
    format!("http://{}{}{}", &api, utils::sep_qs(qs), qs)
}
