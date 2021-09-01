use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::debug;
use reqwest::Client;

use crate::{error, fetch, models, statics, utils};

pub async fn get(
    api: web::Path<String>,
    client: web::Data<Client>,
    req: HttpRequest,
) -> Result<HttpResponse, error::MyError> {
    let api = api.into_inner();
    if api == "favicon.ico" {
        return Ok(HttpResponse::Ok().finish());
    }

    let qs = req.query_string();
    let resource = utils::resource(&api, &qs, "");
    let file_content = utils::read_file(&resource);

    match file_content {
        Ok(value) => Ok(HttpResponse::Ok().body(value)),
        Err(_) => {
            debug!(
                "File not found! For api: {}, resource: {}",
                &api,
                utils::filename(&resource),
            );
            statics::ENVS.allow_externals_calls()?;
            let url = utils::url(&api, &qs);
            let res = fetch::get(&client, &url)
                .await
                .unwrap_or(serde_json::json!({}));
            let file_content = serde_json::to_string(&res).unwrap_or("".to_string());
            utils::write_file(&resource, file_content)?;
            Ok(HttpResponse::Created().json(res))
        }
    }
}

pub async fn post(
    api: web::Path<String>,
    client: web::Data<Client>,
    payload: web::Json<serde_json::Value>,
    req: HttpRequest,
) -> Result<HttpResponse, error::MyError> {
    let api = api.into_inner();
    let payload = payload.into_inner();
    let hash = utils::calculate_hash(&models::HashValue(&payload));

    let qs = req.query_string();
    let resource = utils::resource(&api, &qs, &hash.to_string());
    let file_content = utils::read_file(&resource);

    match file_content {
        Ok(value) => Ok(HttpResponse::Ok().body(value)),
        Err(_) => {
            debug!(
                "File not found! For api: {}, resource: {}, payload_post: {}",
                &api,
                utils::filename(&resource),
                &payload,
            );
            statics::ENVS.allow_externals_calls()?;
            let url = utils::url(&api, &qs);
            let res = fetch::post(&client, &url, &payload)
                .await
                .unwrap_or(serde_json::json!({}));
            let file_content = serde_json::to_string(&res).unwrap_or("".to_string());
            utils::write_file(&resource, file_content)?;
            Ok(HttpResponse::Created().json(res))
        }
    }
}

pub async fn ok() -> Result<impl Responder, error::MyError> {
    Ok(HttpResponse::Ok().body("Ok".to_string()))
}

#[cfg(test)]
mod tests {
    use actix_web::{http, App};

    use super::*;

    #[actix_rt::test]
    async fn test_health() {
        let srv = actix_test::start(|| {
            App::new().service(web::resource("/health").route(web::get().to(ok)))
        });

        assert_eq!(
            srv.get("/health").send().await.unwrap().status(),
            http::StatusCode::OK,
            "Test /health route."
        );
    }

    #[actix_rt::test]
    async fn test_resource_status() {
        let srv = actix_test::start(|| {
            App::new().service(web::resource("/resource-status").route(web::get().to(ok)))
        });

        assert_eq!(
            srv.get("/resource-status").send().await.unwrap().status(),
            http::StatusCode::OK,
            "Test /resource-status route."
        );
    }
}
