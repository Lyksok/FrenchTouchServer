use actix_web::{get, web, HttpResponse, Responder};
use crate::api::run_api::AppState;
use crate::db;

#[get("/users/select/{email}")]
async fn api_select_user_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_email(&*conn, &email) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}",e))),
    }
}
