use actix_web::{post, web, HttpResponse, Responder};

use crate::db;
use crate::db::structs::{
    User,
    Song
};
use crate::api::run_api::AppState;

#[post("/users/insert")]
async fn api_insert_user(
    data: web::Data<AppState>,
    user_data: web::Json<User>
) -> Result<impl Responder, actix_web::Error> {
    println!("/users/insert: json={:?}", &user_data);
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_user(&*conn, &user_data) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}",e))),
    }
}

#[post("/songs/insert")]
async fn api_insert_song(
    data: web::Data<AppState>,
    song_data: web::Json<Song>
) -> Result<impl Responder, actix_web::Error> {
    println!("/songs/insert: json={:?}", &song_data);
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_user(&*conn, &song_data) {
        Ok(song) => Ok(HttpResponse::Ok().json(song)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}",e))),
    }
}
