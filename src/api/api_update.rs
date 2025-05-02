use super::api_utils::UserRequest;
use crate::db;
use crate::{api::run_api::AppState, db::db_security::has_permissions};
use actix_web::{HttpResponse, Responder, post, web};
use rusqlite::Connection;

fn user_changes_himself(conn: &Connection, user_req: UserRequest) -> bool {
    match db::db_select::select_authmap_by_user_id(conn, user_req.obj.id) {
        Some(authmap) => authmap.auth_hash == user_req.auth_hash,
        None => false,
    }
}

#[post("/update/user/profile_picture/{email}")]
async fn api_update_user_profile_picture(
    data: web::Data<AppState>,
    json: web::Json<UserRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = json.auth_hash.clone();
    let user_data = json.obj.clone();
    if !has_permissions(&conn, &user_data, &auth_hash, 0)
        && user_changes_himself(&conn, json.into_inner())
    {
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
    if !has_permissions(&conn, &user_data, &auth_hash, 0)
        && user_changes_himself(&conn, json.into_inner())
    {
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    println!("/users/update: json={:?}", &user_data);
    match db::db_update::update_user_last_connection(&conn, &user_data) {
        Ok(()) => Ok(HttpResponse::Ok().json("")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}
