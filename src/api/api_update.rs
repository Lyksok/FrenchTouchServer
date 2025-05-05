use super::api_utils::{CollaboratorRequestRequest, UserRequest};
use crate::api::api_security::AuthInfo;
use crate::api::api_utils::print_log;
use crate::api::run_api::AppState;
use crate::db;
use crate::db::db_security::{check_password, has_permissions, has_permissions_user};
use crate::db::structs::Credentials;
use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfoNewPassword {
    pub auth_hash: String,
    pub auth_info: AuthInfo,
    pub new_password: String,
}

#[post("/update/user/password/{id}")]
async fn api_update_user_password(
    data: web::Data<AppState>,
    json: web::Json<AuthInfoNewPassword>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_info_new_password = json.into_inner();
    let auth_hash = auth_info_new_password.auth_hash.clone();
    let user_data = auth_info_new_password.auth_info.user.clone();
    if !has_permissions(&conn, &auth_hash, 0)
        && has_permissions_user(&conn, &UserRequest { auth_hash, obj: user_data.clone() })
    {
        print_log("ERROR UPDATE", "User permission", &user_data);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    let cred = match db::db_select::select_credentials_by_user_id(&conn, user_data.id){
        Some(elt)=>elt,
        None=>{
            print_log("ERROR UPDATE", "User credentials", &user_data);
            return Ok(HttpResponse::Forbidden().body("You do not have access"));
        }
    };

    if !check_password(&auth_info_new_password.auth_info.password, &cred.password_salt, &cred.password_hash) {
        print_log("ERROR UPDATE", "User wrong password", &user_data);
        return Ok(HttpResponse::Forbidden().body("You do not have access (password)"));
    }

    let (hash, salt) = db::db_security::new_hash_password(&auth_info_new_password.new_password);
        let cred = Credentials {
            user_id: user_data.id,
            password_hash: hash,
            password_salt: salt,
        };
        let _ = match db::db_update::update_credentials(&conn, &cred) {
            Ok(_) => (),
            Err(_) => {
                print_log("ERROR FTS REGISTER", "Insert Credentials", &user_data);
                return Ok(HttpResponse::Ok().body("User credentials could not be registered"));
            }
        };

    Ok(HttpResponse::Ok().body(""))
}


#[post("/update/collaborator_request/id")]
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

    match db::db_update::update_collaborator_request_by_id(&conn, &json) {
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