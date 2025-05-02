use super::api_utils::{CollaboratorRequestRequest, UserRequest};
use crate::api::api_utils::print_log;
use crate::api::run_api::AppState;
use crate::db;
use crate::db::db_security::{has_permissions, has_permissions_user};
use actix_web::{HttpResponse, Responder, post, web};

#[post("/update/user/profile_picture/{email}")]
async fn api_update_user_profile_picture(
    data: web::Data<AppState>,
    json: web::Json<UserRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let user_data = json.into_inner();
    let auth_hash = user_data.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 0)
        && has_permissions_user(&conn, &user_data)
    {
        print_log("ERROR UPDATE", "User permission", &user_data);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let user_data = user_data.obj.clone();

    match db::db_update::update_user_profile_picture(&conn, &user_data) {
        Ok(()) => {
            print_log("UPDATE", "User profile picture", &user_data);
            Ok(HttpResponse::Ok().json(""))
        }
        Err(e) => {
            print_log("ERROR UPDATE", "User profile picture", &user_data);
            Ok(HttpResponse::InternalServerError().body(e))
        }
    }
}

#[post("/update/user/last_connection/{email}")]
async fn api_update_user_last_connection(
    data: web::Data<AppState>,
    json: web::Json<UserRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let user_data = json.into_inner();
    let auth_hash = user_data.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 0)
        && has_permissions_user(&conn, &user_data)
    {
        print_log("ERROR UPDATE", "User permission", &user_data);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let user_data = user_data.obj.clone();

    match db::db_update::update_user_last_connection(&conn, &user_data) {
        Ok(()) => {
            print_log("UPDATE", "User last connection", &user_data);
            Ok(HttpResponse::Ok().json(""))
        }
        Err(e) => {
            print_log("ERROR UPDATE", "User last connection", &user_data);
            Ok(HttpResponse::InternalServerError().body(e))
        }
    }
}

#[post("/update/collaborator_request/all")]
async fn api_update_collaborator_all(
    data: web::Data<AppState>,
    json: web::Json<CollaboratorRequestRequest>
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let json = json.into_inner();
    let auth_hash = json.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 2)
    {
        print_log("ERROR UPDATE", "User permission", &json);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let json = json.obj.clone();

    match db::db_update::update_collaborator_request_all(&conn, &json) {
        Ok(()) => {
            print_log("UPDATE", "Collaborator Request", &json);
            Ok(HttpResponse::Ok().json(""))
        }
        Err(e) => {
            print_log("ERROR UPDATE", "Collaborator Request", &json);
            Ok(HttpResponse::InternalServerError().body(e))
        }
    }
}