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
    use actix_web::{http, http::header, middleware, web, App};
    use dotenv::dotenv;

    use super::*;

    fn initial_setup() -> Result<(), error::MyError> {
        utils::check_mocks_dir()?;
        Ok(())
    }

    #[actix_rt::test]
    async fn test_health() {
        dotenv().ok();
        initial_setup().ok();

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
        dotenv().ok();
        initial_setup().ok();

        let srv = actix_test::start(|| {
            App::new().service(web::resource("/resource-status").route(web::get().to(ok)))
        });

        assert_eq!(
            srv.get("/resource-status").send().await.unwrap().status(),
            http::StatusCode::OK,
            "Test /resource-status route."
        );
    }

    #[actix_rt::test]
    async fn get_api() {
        dotenv().ok();
        initial_setup().ok();

        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(statics::CLIENT.clone()))
                .wrap(
                    middleware::DefaultHeaders::new()
                        .header(header::SERVER, &statics::ENVS.h_server)
                        .header(header::ACCEPT_CHARSET, mime::UTF_8.to_string())
                        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string()),
                )
                .service(
                    web::resource("/{api:.+}")
                        .route(web::post().to(post))
                        .route(web::get().to(get)),
                )
        });

        assert!(
            srv.get("jsonplaceholder.typicode.com/todos/1")
                .send()
                .await
                .unwrap()
                .status()
                .is_success(),
        );
    }

    #[actix_rt::test]
    async fn post_api() {
        dotenv().ok();
        initial_setup().ok();

        let srv = actix_test::start(move || {
            App::new()
                .app_data(web::Data::new(statics::CLIENT.clone()))
                .wrap(
                    middleware::DefaultHeaders::new()
                        .header(header::SERVER, &statics::ENVS.h_server)
                        .header(header::ACCEPT_CHARSET, mime::UTF_8.to_string())
                        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string()),
                )
                .service(
                    web::resource("/{api:.+}")
                        .route(web::post().to(post))
                        .route(web::get().to(get)),
                )
        });

        let data = r#"{"userId":1,"id":101,"title":"title","body":"body"}"#;
        let json: serde_json::Value = serde_json::from_str(data).unwrap();
        let res = srv
            .post("jsonplaceholder.typicode.com/posts")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .send_json(&json)
            .await
            .unwrap()
            .body()
            .limit(20_000_000)
            .await
            .unwrap();
        let res_json: serde_json::Value =
            serde_json::from_str(&String::from_utf8(res.to_vec()).unwrap()).unwrap();

        assert_eq!(res_json, json);
    }
}
