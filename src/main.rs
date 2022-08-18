use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use etag::EntityTag;
use fullstack_backend::{api, appstate::AppState, db};
use std::env;
use std::sync::RwLock;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to create pool");

    //Create the table if it doesn't exist
    pool.get()
        .unwrap()
        .execute(r#"CREATE TABLE IF NOT EXISTS users (id TEXT, name TEXT, phone TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP);"#, [])
        .expect("Error Creating Table");

    // Note: web::Data created _outside_ HttpServer::new closure
    let state = web::Data::new(AppState {
        is_modified: RwLock::from(true),
        etag: RwLock::new(EntityTag::strong("")),
    });
    log::debug!("{:#?}", state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8082".to_owned())
        .parse::<u16>()
        .expect("COULD NOT PARSE PORT");

    log::info!("starting HTTP server at http://localhost:{}", port);

    HttpServer::new(move || {
        /*  let error_handlers = ErrorHandlers::new()
        .handler(
            http::StatusCode::INTERNAL_SERVER_ERROR,
            api::internal_server_error,
        )
        .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
        .handler(http::StatusCode::NOT_FOUND, api::not_found);*/

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(state.clone())
            //.wrap(error_handlers)
            .service(
                web::scope("/persons")
                    .route("", web::get().to(api::index))
                    .route("", web::post().to(api::add_user))
                    .route("/{id}", web::delete().to(api::delete_user)),
            )
            .service(
                Files::new("/", "./static/build")
                    .use_last_modified(true)
                    .use_etag(true)
                    .index_file("index.html"),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
