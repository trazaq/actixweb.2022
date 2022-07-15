use actix_files::NamedFile;
use actix_web::{dev, error, middleware::ErrorHandlerResponse, web, Error, HttpResponse, Result};
use sqlx::SqlitePool;
use std::ops::Deref;

use crate::model::User;
use crate::{db, AppState};

/*pub async fn index(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let users = db::get_all_users(&pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}*/

pub async fn index(entries: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let entries = entries.phonebook_entries.lock().unwrap(); // <- get phonebook_entries MutexGuard
    Ok(HttpResponse::Ok().json(entries.deref()))
}

/*pub async fn add_user(
    pool: web::Data<SqlitePool>,
    user: web::Json<User>,
) -> Result<HttpResponse, Error> {
    let users = db::add_user(&pool, user.into_inner())
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}*/

pub async fn add_user(
    entries: web::Data<AppState>,
    user: web::Json<User>,
) -> Result<HttpResponse, Error> {
    let mut entries = entries.phonebook_entries.lock().unwrap(); // <- get phonebook_entries MutexGuard
    entries.push(user.clone());

    Ok(HttpResponse::Ok().json(user.into_inner()))
}

/*pub async fn delete_user(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    db::delete_user(&pool, id)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
*/

pub async fn delete_user(
    entries: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let mut entries = entries.phonebook_entries.lock().unwrap(); // <- get phonebook_entries MutexGuard
    let id = path.into_inner();
    let index =
        entries
            .iter()
            .position(|user| user.id == id)
            .ok_or_else(|| error::ErrorInternalServerError(
                "Couldn't find entry to delete",
            ));
    if let Ok(i) = index {
        entries.remove(i);
    }
    Ok(HttpResponse::Ok().finish())
}

/*#[derive(Deserialize)]
pub struct CreateForm {
    description: String,
}

pub async fn create(
    params: web::Form<CreateForm>,
    pool: web::Data<SqlitePool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    if params.description.is_empty() {
        session::set_flash(&session, FlashMessage::error("Description cannot be empty"))?;
        Ok(redirect_to("/"))
    } else {
        db::create_task(params.into_inner().description, &pool)
            .await
            .map_err(error::ErrorInternalServerError)?;
        session::set_flash(&session, FlashMessage::success("Task successfully added"))?;
        Ok(redirect_to("/"))
    }
}

#[derive(Deserialize)]
pub struct UpdateParams {
    id: i32,
}

#[derive(Deserialize)]
pub struct UpdateForm {
    _method: String,
}

pub async fn update(
    db: web::Data<SqlitePool>,
    params: web::Path<UpdateParams>,
    form: web::Form<UpdateForm>,
    session: Session,
) -> Result<HttpResponse, Error> {
    match form._method.as_ref() {
        "put" => toggle(db, params).await,
        "delete" => delete(db, params, session).await,
        unsupported_method => {
            let msg = format!("Unsupported HTTP method: {unsupported_method}");
            Err(error::ErrorBadRequest(msg))
        }
    }
}

async fn toggle(
    pool: web::Data<SqlitePool>,
    params: web::Path<UpdateParams>,
) -> Result<HttpResponse, Error> {
    db::toggle_task(params.id, &pool)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(redirect_to("/"))
}

async fn delete(
    pool: web::Data<SqlitePool>,
    params: web::Path<UpdateParams>,
    session: Session,
) -> Result<HttpResponse, Error> {
    db::delete_task(params.id, &pool)
        .await
        .map_err(error::ErrorInternalServerError)?;
    session::set_flash(&session, FlashMessage::success("Task was deleted."))?;
    Ok(redirect_to("/"))
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, location))
        .finish()
}
*/
pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}
