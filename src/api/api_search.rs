use actix_web::{get, web, HttpResponse, Responder};

use crate::db;

use super::{api_utils::print_log, run_api::AppState};

#[get("/select/search/song/{title}")]
async fn api_select_search_song(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_search::select_search_song(&conn, &title){
        Some(elt) => {
            print_log("SELECT", "Search song", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        None => {
            print_log("ERROR SELECT", "Search song", &title);
            Ok(HttpResponse::InternalServerError().body("Could not find songs"))
        }
    }
}

#[get("/select/search/artist/{name}")]
async fn api_select_search_artist(
    data: web::Data<AppState>,
    name: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_search::select_search_artist(&conn, &name){
        Some(elt) => {
            print_log("SELECT", "Search artist", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        None => {
            print_log("ERROR SELECT", "Search artist", &name);
            Ok(HttpResponse::InternalServerError().body("Could not find artists"))
        }
    }
}