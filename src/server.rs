use actix_files::Files;
use actix_web::{http::header, middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::{info, warn};
use mime;

use crate::{handlers, statics, utils};

pub async fn new() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let envs = utils::envs();
    let localhost = envs.localhost.clone();
    let user_agent = envs.user_agent.clone();

    utils::check_mocks_dir()?;

    info!("Server running in {}", localhost);
    warn!(
        "Calling externals apis is allowed? {:#?}",
        envs.allow_externals
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(statics::CLIENT.clone()))
            .app_data(web::Data::new(envs.clone()))
            .wrap(
                middleware::DefaultHeaders::new()
                    .header(header::USER_AGENT, user_agent.clone())
                    .header(header::ACCEPT_CHARSET, mime::UTF_8.to_string())
                    .header(
                        header::CONTENT_TYPE,
                        mime::APPLICATION_JAVASCRIPT.to_string(),
                    ),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::new("%s %T %r %{User-Agent}i bytes:%b"))
            .service(Files::new("/mocks", format!("{}/", statics::MOCK_DIR)).show_files_listing())
            .service(web::resource("/status").route(web::get().to(handlers::status)))
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
