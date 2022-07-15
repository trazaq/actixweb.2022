mod api;
mod db;
mod model;

use crate::model::User;
use actix_files::Files;
use actix_web::middleware::{ErrorHandlers, Logger};
use actix_web::{get, http, middleware, web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;

#[get("/persons")]
async fn greet() -> impl Responder {
    actix_files::NamedFile::open_async("./db.json").await
}

#[derive(Debug)]
pub struct AppState {
    phonebook_entries: Mutex<Vec<User>>,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to create pool");

    // Note: web::Data created _outside_ HttpServer::new closure
    let entries = web::Data::new(AppState {
        phonebook_entries: Mutex::new(
            db::get_all_users(&pool)
                .await
                .expect("Error Setting AppData Users"),
        ),
    });
    //log::info!("{:#?}", entries);

    let port = std::env::var("PORT")
        .expect("ENV PORT NOT SET")
        .parse::<u16>()
        .expect("COULD NOT PARSE PORT");

    log::info!("starting HTTP server at http://localhost:{}", port);

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
            .app_data(entries.clone())
            .wrap(error_handlers)
            .service(
                web::scope("/persons")
                    .route("", web::get().to(api::index))
                    .route("", web::post().to(api::add_user))
                    .route("/{id}", web::delete().to(api::delete_user)),
            )
            .service(Files::new("/", "./static/build").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
