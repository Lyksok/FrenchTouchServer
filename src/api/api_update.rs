use crate::db;
use crate::{api::run_api::AppState, db::db_security::has_permissions};
use actix_web::{post, web, HttpResponse, Responder};
use super::api_utils::UserRequest; 

#[post("/update/user/profile_picture/{email}")]
async fn api_update_user_profile_picture(
    data: web::Data<AppState>,
    json: web::Json<UserRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = json.auth_hash.clone();
    let user_data = json.obj.clone();
    if !has_permissions(&conn, &user_data, &auth_hash, 0) {
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    println!("/users/update: json={:?}", &user_data);
    match db::db_update::update_user_profile_picture(&conn, &user_data) {
        Ok(()) => Ok(HttpResponse::Ok().json("")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

#[post("/update/user/last_connection/{email}")]
async fn api_update_user_last_connection(
    data: web::Data<AppState>,
    json: web::Json<UserRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = json.auth_hash.clone();
    let user_data = json.obj.clone();
    if !has_permissions(&conn, &user_data, &auth_hash, 0) {
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    println!("/users/update: json={:?}", &user_data);
    match db::db_update::update_user_last_connection(&conn, &user_data) {
        Ok(()) => Ok(HttpResponse::Ok().json("")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}
