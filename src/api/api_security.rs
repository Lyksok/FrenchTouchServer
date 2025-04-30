use crate::api::api_utils::print_log;
use crate::db;
use crate::db::db_exist::{
    admin_exist_by_user_id, artist_exist_by_user_id, authmap_exist_by_hash,
    authmap_exist_by_user_id, collaborator_exist_by_user_id,
};
use crate::{api::run_api::AppState, db::structs::AuthMap};
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct AuthInfo {
    user_id: i64,
    password: String,
}

#[get("/fts_login")]
async fn api_security_login(
    data: web::Data<AppState>,
    auth_info: web::Json<AuthInfo>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    // Check if user already have authenticate certificate and return it
    if authmap_exist_by_user_id(&conn, auth_info.user_id) {
        let authmap = db::db_select::select_authmap_by_user_id(&conn, auth_info.user_id).unwrap();
        print_log("FTS LOGIN", "AuthMap", &authmap);
        return Ok(HttpResponse::Ok().json(format!("{{auth_hash:{}}}", authmap.auth_hash)));
    }

    // Create a unique uuid hash
    let mut auth_hash = None;
    while auth_hash.is_none() {
        auth_hash = match Uuid::new_v4() {
            uuid if authmap_exist_by_hash(&conn, &format!("{}", uuid)) => None,
            uuid => Some(uuid.hyphenated()),
        }
    }

    // Else create one and return it
    match db::db_exist::user_exist_by_id(&conn, auth_info.user_id) {
        true => {
            let _ = match db::db_insert::insert_authmap(
                &conn,
                &AuthMap {
                    user_id: auth_info.user_id,
                    auth_hash: format!(
                        "{}",
                        match auth_hash {
                            Some(h) => h,
                            None => {
                                print_log(
                                    "ERROR FTS LOGIN",
                                    "Hash error user id",
                                    &auth_info.user_id,
                                );
                                return Ok(
                                    HttpResponse::InternalServerError().body("Could not get hash")
                                );
                            }
                        }
                    ),
                    permission_level: match auth_info.user_id {
                        id if admin_exist_by_user_id(&conn, id) => 3,
                        id if collaborator_exist_by_user_id(&conn, id) => 2,
                        id if artist_exist_by_user_id(&conn, id) => 1,
                        _ => 0,
                    },
                },
            ) {
                Some(id) => id,
                None => {
                    print_log("ERROR FTS LOGIN", "Insert hash user id", &auth_info.user_id);
                    return Ok(HttpResponse::InternalServerError().body("Could not save hash"));
                }
            };

            let auth_map = match db::db_select::select_authmap_by_user_id(&conn, auth_info.user_id)
            {
                Some(authmap) => authmap,
                None => {
                    print_log(
                        "ERROR FTS LOGIN",
                        "HashMap not found user id",
                        &auth_info.user_id,
                    );
                    return Ok(HttpResponse::InternalServerError().body("Could not select hash"));
                }
            };

            print_log("FTS LOGIN", "AuthMap", &auth_map);
            Ok(HttpResponse::Ok().json(format!("{{auth_hash:{}}}", auth_map.auth_hash)))
        }
        false => {
            print_log("ERROR FTS LOGIN", "User id", &auth_info.user_id);
            Ok(HttpResponse::InternalServerError().body("User does not exist"))
        }
    }
}

#[get("/fts_register")]
async fn api_security_register(
    data: web::Data<AppState>,
    auth_info: web::Json<AuthInfo>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    // Check if user already exists
    if authmap_exist_by_user_id(&conn, auth_info.user_id) {
        let authmap = db::db_select::select_authmap_by_user_id(&conn, auth_info.user_id).unwrap();
        print_log("ERROR FTS REGISTER", "AuthMap", &authmap);
        return Ok(HttpResponse::Ok().body("User already registered"));
    }

    // Create a unique uuid hash
    let mut auth_hash = None;
    while auth_hash.is_none() {
        auth_hash = match Uuid::new_v4() {
            uuid if authmap_exist_by_hash(&conn, &format!("{}", uuid)) => None,
            uuid => Some(uuid.hyphenated()),
        }
    }

    // Else create one and return it
    match db::db_exist::user_exist_by_id(&conn, auth_info.user_id) {
        true => {
            let _ = match db::db_insert::insert_authmap(
                &conn,
                &AuthMap {
                    user_id: auth_info.user_id,
                    auth_hash: format!(
                        "{}",
                        match auth_hash {
                            Some(h) => h,
                            None => {
                                print_log(
                                    "ERROR FTS LOGIN",
                                    "Hash error user id",
                                    &auth_info.user_id,
                                );
                                return Ok(
                                    HttpResponse::InternalServerError().body("Could not get hash")
                                );
                            }
                        }
                    ),
                    permission_level: match auth_info.user_id {
                        id if admin_exist_by_user_id(&conn, id) => 3,
                        id if collaborator_exist_by_user_id(&conn, id) => 2,
                        id if artist_exist_by_user_id(&conn, id) => 1,
                        _ => 0,
                    },
                },
            ) {
                Some(id) => id,
                None => {
                    print_log("ERROR FTS LOGIN", "Insert hash user id", &auth_info.user_id);
                    return Ok(HttpResponse::InternalServerError().body("Could not save hash"));
                }
            };

            let auth_map = match db::db_select::select_authmap_by_user_id(&conn, auth_info.user_id)
            {
                Some(authmap) => authmap,
                None => {
                    print_log(
                        "ERROR FTS LOGIN",
                        "HashMap not found user id",
                        &auth_info.user_id,
                    );
                    return Ok(HttpResponse::InternalServerError().body("Could not select hash"));
                }
            };

            print_log("FTS LOGIN", "AuthMap", &auth_map);
            Ok(HttpResponse::Ok().json(format!("{{auth_hash:{}}}", auth_map.auth_hash)))
        }
        false => {
            print_log("ERROR FTS REGISTER", "User id", &auth_info.user_id);
            Ok(HttpResponse::InternalServerError().body("User does not exist"))
        }
    }
}
