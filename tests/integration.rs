use actix_web::{http::header, test, web, App};
use etag::EntityTag;
use fullstack_backend::api::{add_user, index};
use fullstack_backend::appstate::AppState;
use fullstack_backend::model::User;
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
