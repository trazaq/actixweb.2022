mod api;
mod db;
mod model;

use actix_files::Files;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{get, middleware, web, http, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/persons")]
async fn greet() -> impl Responder {
    actix_files::NamedFile::open_async("./db.json").await
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to create pool");

    log::info!("starting HTTP server at http://localhost:8082");

    HttpServer::new(move || {
        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                api::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
            .handler(http::StatusCode::NOT_FOUND, api::not_found);

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .wrap(error_handlers)
            .service(web::resource("/persons").route(web::get().to(api::index)))
            .service(Files::new("/", "./static/build").index_file("index.html"))

    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
