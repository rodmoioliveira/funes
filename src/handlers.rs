use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::{Client, Method};

use crate::{api, error, fetch, format, io, statics, utils};

pub async fn get(
    api: web::Path<String>,
    client: web::Data<Client>,
    req: HttpRequest,
) -> Result<HttpResponse, error::FunesError> {
    let api = api.into_inner();
    if api == "favicon.ico" {
        return Ok(HttpResponse::Ok().finish());
    }

    let qs = req.query_string();
    let resource = format::resource(&api, qs, "");
    let file_content = io::read(&resource);

    api::sleep(&api, &statics::LATENCY_COLLECTION).await?;

    match file_content {
        Ok((status, body)) => Ok(HttpResponse::build(status).json(body)),
        Err(_) => {
            statics::ENVS.allow_externals_calls()?;

            let url = format::url(&api, qs);
            let res: reqwest::Response = fetch::get(&client, &url).await?;

            let (status, body) = io::write(&resource, res, Method::GET, None).await?;
            Ok(HttpResponse::build(status).json(body))
        }
    }
}

pub async fn post(
    api: web::Path<String>,
    client: web::Data<Client>,
    payload: web::Json<serde_json::Value>,
    req: HttpRequest,
) -> Result<HttpResponse, error::FunesError> {
    let api = api.into_inner();
    let payload = payload.into_inner();
    let hash = utils::hash(&utils::HashValue(&payload));

    let qs = req.query_string();
    let resource = format::resource(&api, qs, &hash.to_string());
    let file_content = io::read(&resource);

    api::sleep(&api, &statics::LATENCY_COLLECTION).await?;

    match file_content {
        Ok((status, body)) => Ok(HttpResponse::build(status).json(body)),
        Err(_) => {
            statics::ENVS.allow_externals_calls()?;

            let url = format::url(&api, qs);
            let res: reqwest::Response = fetch::post(&client, &url, &payload).await?;

            let (status, body) = io::write(&resource, res, Method::POST, Some(&payload)).await?;
            Ok(HttpResponse::build(status).json(body))
        }
    }
}

pub async fn ok() -> Result<impl Responder, error::FunesError> {
    Ok(HttpResponse::Ok().body("Ok".to_string()))
}

#[cfg(test)]
mod tests {
    use actix_web::{http, http::header, middleware, web, App};
    use dotenv::dotenv;

    use super::*;

    fn initial_setup() -> Result<(), error::FunesError> {
        io::mock_dir()?;
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

        let mut res = srv
            .get("jsonplaceholder.typicode.com/todos/1")
            .send()
            .await
            .unwrap();
        assert!(res.status().is_success(),);

        let data = r#"{"completed":false,"id":1,"title":"delectus aut autem","userId":1}"#;
        let json: serde_json::Value = serde_json::from_str(data).unwrap();
        let res_json: serde_json::Value = serde_json::from_str(
            &String::from_utf8(res.body().limit(20_000_000).await.unwrap().to_vec()).unwrap(),
        )
        .unwrap();
        assert_eq!(res_json, json);
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
