use crate::db;
use crate::db::db_exist::{
    admin_exist_by_user_id, artist_exist_by_user_id, authmap_exist_by_hash,
    authmap_exist_by_user_id, collaborator_exist_by_user_id,
};
use crate::{
    api::run_api::AppState,
    db::structs::{AuthMap, User},
};
use actix_web::{get, web, HttpResponse, Responder};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[get("/fts-login")]
async fn api_security_login(
    data: web::Data<AppState>,
    user_cred: web::Json<User>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    // Check if user already have authenticate certificate and return it
    if authmap_exist_by_user_id(&conn, user_cred.id) {
        return Ok(HttpResponse::Ok().json(format!(
            "{{auth_hash:{}}}",
            db::db_select::select_authmap_by_user_id(&conn, user_cred.id)
                .unwrap()
                .auth_hash
        )));
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
    match db::db_exist::user_exist_by_email(&conn, &user_cred.email) {
        true => {
            let auth_id;
            // Check if admin/artist/collab/user
            if admin_exist_by_user_id(&conn, user_cred.id) {
                auth_id = db::db_insert::insert_authmap(
                    &conn,
                    &AuthMap {
                        id: -1,
                        user_id: user_cred.id,
                        auth_hash: format!("{}", auth_hash.unwrap()),
                        permission_level: 3,
                        expiration_date: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i32
                            + 3600,
                    },
                )
                .unwrap();
            } else if artist_exist_by_user_id(&conn, user_cred.id) {
                auth_id = db::db_insert::insert_authmap(
                    &conn,
                    &AuthMap {
                        id: -1,
                        user_id: user_cred.id,
                        auth_hash: format!("{}", auth_hash.unwrap()),
                        permission_level: 1,
                        expiration_date: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i32
                            + 3600,
                    },
                )
                .unwrap();
            } else if collaborator_exist_by_user_id(&conn, user_cred.id) {
                auth_id = db::db_insert::insert_authmap(
                    &conn,
                    &AuthMap {
                        id: -1,
                        user_id: user_cred.id,
                        auth_hash: format!("{}", auth_hash.unwrap()),
                        permission_level: 2,
                        expiration_date: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i32
                            + 3600,
                    },
                )
                .unwrap();
            } else {
                auth_id = db::db_insert::insert_authmap(
                    &conn,
                    &AuthMap {
                        id: -1,
                        user_id: user_cred.id,
                        auth_hash: format!("{}", auth_hash.unwrap()),
                        permission_level: 0,
                        expiration_date: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i32
                            + 3600,
                    },
                )
                .unwrap();
            }

            let auth_hash = db::db_select::select_authmap_by_id(&conn, auth_id).unwrap();

            println!("[CRED] User {:?}", user_cred);
            Ok(HttpResponse::Ok().json(format!("{{auth_hash:{}}}", auth_hash.auth_hash)))
        }
        false => {
            println!("[CRED ERROR] Could not authenticate user {:?}", user_cred);
            Ok(HttpResponse::InternalServerError().body("Could not authenticate user."))
        }
    }
}
