use actix_web::{http::header, test, web, App};
use dotenv::dotenv;
use etag::EntityTag;
use fullstack_backend::api::{add_user, index};
use fullstack_backend::appstate::AppState;
use fullstack_backend::db;
use fullstack_backend::model::User;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::env;
use std::sync::RwLock;

#[actix_web::test]
async fn test_index_ok() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL_DEV").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to create pool");

    //Create the table if it doesn't exist
    pool.get()
        .unwrap()
        .execute(r#"CREATE TABLE IF NOT EXISTS users (id TEXT, name TEXT, phone TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP);"#, [])
        .expect("Error Creating Table");

    //Insert test user
    pool.get()
        .unwrap()
        .execute(
            r#"INSERT OR REPLACE INTO users (id, name, phone) VALUES ('1232', 'TEST USER', '999-999-9999');"#,
            [],
        )
        .expect("Error Inserting Users into DB");

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(AppState {
                is_modified: RwLock::from(true),
                etag: RwLock::new(EntityTag::new(false, "")),
            }))
            .route("/", web::get().to(index))
            .route("/persons", web::post().to(add_user)),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp: Vec<User> = test::call_and_read_body_json(&app, req).await;
    assert!(resp.len() >= 1, "No Users returned from the Database!");
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
