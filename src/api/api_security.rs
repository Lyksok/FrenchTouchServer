use crate::api::api_utils::print_log;
use crate::db::db_exist::{authmap_exist_by_hash, user_exist_by_id};
use crate::db::db_security::{check_password, new_hash_password};
use crate::db::structs::{Credentials, User};
use crate::db::{self, db_insert, db_select};
use crate::{api::run_api::AppState, db::structs::AuthMap};
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct AuthInfo {
    user: User,
    password: String,
}

#[get("/fts_login")]
async fn api_security_login(
    data: web::Data<AppState>,
    auth_info: web::Json<AuthInfo>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    // Check if user is correct (id != -1,id maps to existing user)
    // If user incorrect -> error
    let user = auth_info.user.clone();
    if user.id == -1 || !user_exist_by_id(&conn, user.id) {
        print_log("ERROR FTS LOGIN", "User", &user);
        Ok(HttpResponse::InternalServerError().body("User is not valid"))
    }
    // If user correct -> check credentials
    else {
        let cred = match db::db_select::select_credentials_by_user_id(&conn, user.id) {
            Some(elt) => elt,
            None => {
                print_log("ERROR FTS LOGIN", "User Credentials", &user);
                return Ok(HttpResponse::InternalServerError().body("Credentials do not exist"));
            }
        };
        if !check_password(
            &auth_info.password,
            &cred.password_salt,
            &cred.password_hash,
        ) {
            print_log("ERROR FTS LOGIN", "User password", &user);
            return Ok(HttpResponse::BadRequest().body("Wrong password"));
        }

        let authmap = match db::db_select::select_authmap_by_user_id(&conn, user.id) {
            Some(elt) => elt,
            None => {
                print_log("ERROR FTS LOGIN", "User", &user);
                return Ok(HttpResponse::InternalServerError().body("Hash does not exist"));
            }
        };
        print_log("FTS LOGIN", "AuthMap", &authmap);
        Ok(HttpResponse::Ok().json(format!("{{auth_hash:{}}}", authmap.auth_hash)))
    }
}

#[get("/fts_register")]
async fn api_security_register(
    data: web::Data<AppState>,
    auth_info: web::Json<AuthInfo>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let mut user = auth_info.user.clone();

    // Check if user is correct (id == -1)
    if user.id != -1 {
        print_log("ERROR FTS REGISTER", "User", &user);
        return Ok(HttpResponse::Ok().body("User already registered"));
    }
    // Create user
    let new_id = match db_insert::insert_user(&conn, &user) {
        Some(id) => id,
        None => {
            print_log("ERROR FTS REGISTER", "Insert User", &user);
            return Ok(HttpResponse::Ok().body("User could not be registered"));
        }
    };
    user.id = new_id;

    // Create credentials
    {
        let (hash, salt) = new_hash_password(&auth_info.password);
        let cred = Credentials {
            user_id: user.id,
            password_hash: hash,
            password_salt: salt,
        };
        match db_insert::insert_credentials(&conn, &cred) {
            Some(_) => (),
            None => {
                print_log("ERROR FTS REGISTER", "Insert Credentials", &user);
                return Ok(HttpResponse::Ok().body("User credentials could not be registered"));
            }
        }
    }
    // Create auth_hash
    let mut auth_hash = None;
    while auth_hash.is_none() {
        auth_hash = match Uuid::new_v4() {
            uuid if authmap_exist_by_hash(&conn, &format!("{}", uuid)) => None,
            uuid => Some(uuid.hyphenated()),
        }
    }
    let auth_hash = match auth_hash {
        Some(h) => h,
        None => {
            print_log("ERROR FTS REGISTER", "User hash", &user.id);
            return Ok(HttpResponse::Ok().body("User hash could not be created"));
        }
    };
    let authmap = AuthMap {
        user_id: user.id,
        auth_hash: format!("{}", auth_hash),
        permission_level: 0,
    };
    match db_insert::insert_authmap(&conn, &authmap) {
        Some(_) => (),
        None => {
            print_log("ERROR FTS REGISTER", "Insert AuthMap", &user);
            return Ok(HttpResponse::Ok().body("User AuthMap could not be registered"));
        }
    }
    match db_select::select_authmap_by_user_id(&conn, user.id) {
        Some(authmap) => {
            print_log("FTS REGISTER", "User AutentityentityhMap", &authmap);
            Ok(HttpResponse::Ok().json(format!("{{auth_hash:{}}}", authmap.auth_hash)))
        }
        None => {
            print_log("ERROR FTS REGISTER", "User AuthMap", &user);
            Ok(HttpResponse::Ok().body("User hash could not be retrieved"))
        }
    }
}
