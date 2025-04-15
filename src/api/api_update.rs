use actix_web::{post, web, HttpResponse, Responder};

use crate::api::run_api::AppState;
use crate::db;
use crate::db::structs::User;

#[post("/update/user/profile-picture/{email}")]
async fn api_update_user_profile_picture(
    data: web::Data<AppState>,
    user_data: web::Json<User>,
) -> Result<impl Responder, actix_web::Error> {
    println!("/users/update: json={:?}", &user_data);
    let conn = data.db.lock().unwrap();
    match db::db_update::update_user_profile_picture(&conn, &user_data) {
        Ok(()) => Ok(HttpResponse::Ok().json("")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    }
}
