use crate::appstate::AppState;
use crate::db;
use crate::model::User;
use actix_files::NamedFile;
use actix_web::http::header::{ETag, EntityTag};
use actix_web::http::{header, StatusCode};
use actix_web::{
    dev, middleware::ErrorHandlerResponse, web, Error, HttpRequest, HttpResponse, Result,
};
use etag::EntityTag as OtherEntityTag;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rayon::prelude::*;

/*pub async fn index(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let users = db::get_all_users(&pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}*/

pub async fn index(
    req: HttpRequest,
    state: web::Data<AppState>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    log::debug!("{:#?}", req.headers());
    let users = db::get_all_users(&pool).await.unwrap();
    //dereference instead of making a clone, so it's a copy type
    //actix won't respond if you don't clone or copy because you're reading it in the if condition below,
    //then trying to modify it in the body. it compiles, but hangs
    let is_modified = *state.is_modified.read().unwrap();

    // First check if the content has changed and if so, calculate the new etag and update the shared state's etag to it
    if is_modified {
        let updated = OtherEntityTag::from_data(
            users
                .par_iter()
                .map(|v| serde_json::to_string(v).unwrap())
                .collect::<Vec<String>>()
                .join("")
                .as_ref(),
        );
        *state.etag.write().unwrap() = updated;
        *state.is_modified.write().unwrap() = false; //reset to false
    };

    let tag = state.etag.read().unwrap();
    log::debug!("Tag {:#?}", tag.to_string());

    // If there's a if-none-match header value and it matches the etag calculation of the above data,
    // the return a 304 response. Under other circumstances return a regular 200 response with the contents.
    // This is to save bandwidth, but may become problematic if the data is too large?
    if let Some(etag) = req.headers().get(header::IF_NONE_MATCH) {
        return match etag.to_str() {
            Ok(etag) if etag == tag.to_string() => {
                Ok(HttpResponse::Ok().status(StatusCode::NOT_MODIFIED).finish())
            }
            _ => Ok(HttpResponse::Ok()
                .insert_header(ETag(EntityTag::new_strong(tag.tag().into())))
                .json(users)),
        };
    } else {
        Ok(HttpResponse::Ok()
            .insert_header(ETag(EntityTag::new_strong(tag.tag().into())))
            .json(users))
    }
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
    state: web::Data<AppState>,
    user: web::Json<User>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    match db::add_user(&pool, user.into_inner()).await {
        Ok(u) => {
            log::info!("User Added: {:#?}", u);
            *state.is_modified.write().unwrap() = true; //to let the index func know to calculate the etag since content has changed
            Ok(HttpResponse::Ok().json(u))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
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
    state: web::Data<AppState>,
    path: web::Path<String>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    if let Ok(()) = db::delete_user(&pool, &id).await {
        log::info!("User w/ ID {:#?} Deleted", id);
        *state.is_modified.write().unwrap() = true; //to let the index func know to calculate the etag since content has changed
    } else {
        return Ok(HttpResponse::NotFound().body("No Entry Found"));
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
pub fn _bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn _not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn _internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}
