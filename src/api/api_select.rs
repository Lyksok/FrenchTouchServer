use crate::api::run_api::AppState;
use crate::db;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/select/user/email/{email}")]
async fn api_select_user_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_email(&conn, &email) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the user")),
    }
}
