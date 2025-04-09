use actix_web::{post, web, HttpResponse, Responder};

use crate::db;
use crate::db::structs::User;
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
