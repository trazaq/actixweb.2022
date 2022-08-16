mod api;
mod db;
mod model;

use crate::model::User;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{get, middleware, web, App, HttpServer, Responder};
use dotenv::dotenv;
use etag::EntityTag;
use std::env;
use std::sync::RwLock;

#[get("/persons")]
async fn greet() -> impl Responder {
    actix_files::NamedFile::open_async("./db.json").await
}

#[derive(Debug)]
pub struct AppState {
    phonebook_entries: RwLock<Vec<User>>,
    is_modified: RwLock<bool>,
    etag: RwLock<EntityTag>,
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
        phonebook_entries: RwLock::new(
            db::get_all_users(&pool)
                .await
                .expect("Error Setting AppData Users"),
        ),
        is_modified: RwLock::from(true),
        etag: RwLock::new(EntityTag::strong("")),
    });
    log::debug!("{:#?}", entries);

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
            .app_data(entries.clone())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{add_user, index};
    use actix_web::{http::header, test, App};
    use serde_json::json;
    use std::sync::RwLock;

    #[actix_web::test]
    async fn test_index_ok() {
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    phonebook_entries: RwLock::new(vec![User {
                        id: "1232".to_string(),
                        name: "TEST USER".to_string(),
                        phone: "999-999-9999".to_string(),
                    }]),
                    is_modified: RwLock::from(true),
                    etag: RwLock::new(EntityTag::new(false, "")),
                }))
                .route("/", web::get().to(index))
                .route("/persons", web::post().to(add_user)),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp: Vec<User> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(resp[0].id, "1232");

        let payload = json!({"id":"09876-0-900", "name":"New User", "phone":"999-999-9999" });
        let add_user_req = test::TestRequest::post()
            .uri("/persons")
            .insert_header(header::ContentType::json())
            .set_payload(payload.to_string().as_bytes().to_owned())
            .to_request();
        let add_user_resp: User = test::call_and_read_body_json(&mut app, add_user_req).await;
        assert_eq!(json!(add_user_resp), payload);
    }
}
