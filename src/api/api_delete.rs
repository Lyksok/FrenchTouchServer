use actix_web::{post, web, HttpResponse, Responder};

use crate::{api::api_utils::{print_log, AuthHash}, db::{self, db_security::has_permissions}};

use super::run_api::AppState;

#[post("/delete/request_to_collaborator/user_id/{user_id}")]
async fn api_delete_request_to_collaborator_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR DELETE", "User permission (RequestToCollaborator)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_request_to_collaborator_by_user_id(&conn, *user_id) {
        true => {
            print_log("DELETE", "RequestToCollaborator", &user_id);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "RequestToCollaborator", &user_id);
            return Ok(HttpResponse::InternalServerError().body("Could not delete RequestToCollaborator"))
        },
    }
}

#[post("/delete/request_to_artist/user_id/{user_id}")]
async fn api_delete_request_to_artist_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR DELETE", "User permission (RequestToArtist)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_request_to_artist_by_user_id(&conn, *user_id) {
        true => {
            print_log("DELETE", "RequestToArtist", &user_id);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "RequestToArtist", &user_id);
            return Ok(HttpResponse::InternalServerError().body("Could not delete RequestToArtist"))
        },
    }
}

#[post("/delete/request_to_admin/user_id/{user_id}")]
async fn api_delete_request_to_admin_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR DELETE", "User permission (RequestToAdmin)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_request_to_admin_by_user_id(&conn, *user_id) {
        true => {
            print_log("DELETE", "RequestToAdmin", &user_id);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "RequestToAdmin", &user_id);
            return Ok(HttpResponse::InternalServerError().body("Could not delete RequestToAdmin"))
        },
    }
}