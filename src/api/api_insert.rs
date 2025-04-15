use actix_web::{post, web, HttpResponse, Responder};

use crate::api::run_api::AppState;
use crate::db;
use crate::db::structs::{Song, User};

#[post("/users/insert")]
async fn api_insert_user(
    data: web::Data<AppState>,
    user_data: web::Json<User>,
) -> Result<impl Responder, actix_web::Error> {
    println!("/users/insert: json={:?}", &user_data);
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_user(&conn, &user_data) {
        true => {
            Ok(HttpResponse::Ok()
                .json(db::db_select::select_user_by_email(&conn, &user_data.email)))
        }
        false => Ok(HttpResponse::InternalServerError().body(format!("Could not insert user."))),
    }
}

#[post("/songs/insert")]
async fn api_insert_song(
    data: web::Data<AppState>,
    song_data: web::Json<Song>,
) -> Result<impl Responder, actix_web::Error> {
    println!("/songs/insert: json={:?}", &song_data);
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_song(&conn, &song_data) {
        Ok(song) => Ok(HttpResponse::Ok().json(song)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    }
}
