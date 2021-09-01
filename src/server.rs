use actix_files::Files;
use actix_web::{http::header, middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use mime;

use crate::{handlers, io, statics};

pub async fn new() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    io::check_mocks_dir()?;

    let localhost = &statics::ENVS.localhost;
    info!("Server running in {}", localhost);
    info!("Mocks directory is {}", statics::ENVS.mock_dir);
    info!(
        "Calling externals apis is allowed? {:#?}",
        statics::ENVS.allow_externals
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(statics::CLIENT.clone()))
            .wrap(
                middleware::DefaultHeaders::new()
                    .header(header::SERVER, &statics::ENVS.h_server)
                    .header(header::ACCEPT_CHARSET, mime::UTF_8.to_string())
                    .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string()),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::new("%s %T %r %{User-Agent}i bytes:%b"))
            .service(
                Files::new("/mocks", format!("{}/", &statics::ENVS.mock_dir)).show_files_listing(),
            )
            .service(web::resource("/health").route(web::get().to(handlers::ok)))
            .service(web::resource("/resource-status").route(web::get().to(handlers::ok)))
            .service(
                web::resource("/{api:.+}")
                    .route(web::post().to(handlers::post))
                    .route(web::get().to(handlers::get)),
            )
    })
    .keep_alive(3600)
    .bind(localhost)?
    .run()
    .await
}
