use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db;

use super::{api_utils::print_log, run_api::AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchPayload {
    str: String,
}

#[post("/select/search/song")]
async fn api_select_search_song(
    data: web::Data<AppState>,
    payload: web::Json<SearchPayload>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let title = payload.str.clone();
    match db::db_search::select_search_song(&conn, &title){
        Some(elt) => {
            print_log("SELECT", "Search song", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        None => {
            print_log("ERROR SELECT", "Search song", &title);
            Ok(HttpResponse::InternalServerError().body("Could not find song"))
        }
    }
}

#[post("/select/search/artist")]
async fn api_select_search_artist(
    data: web::Data<AppState>,
    payload: web::Json<SearchPayload>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let name = payload.str.clone();
    match db::db_search::select_search_artist(&conn, &name){
        Some(elt) => {
            print_log("SELECT", "Search artist", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        None => {
            print_log("ERROR SELECT", "Search artist", &name);
            Ok(HttpResponse::InternalServerError().body("Could not find artist"))
        }
    }
}

#[post("/select/search/album")]
async fn api_select_search_album(
    data: web::Data<AppState>,
    payload: web::Json<SearchPayload>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let name = payload.str.clone();
    match db::db_search::select_search_album(&conn, &name){
        Some(elt) => {
            print_log("SELECT", "Search album", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        None => {
            print_log("ERROR SELECT", "Search album", &name);
            Ok(HttpResponse::InternalServerError().body("Could not find album"))
        }
    }
}

#[post("/select/search/playlist")]
async fn api_select_search_playlist(
    data: web::Data<AppState>,
    payload: web::Json<SearchPayload>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let name = payload.str.clone();
    match db::db_search::select_search_playlist(&conn, &name){
        Some(elt) => {
            print_log("SELECT", "Search playlist", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        None => {
            print_log("ERROR SELECT", "Search playlist", &name);
            Ok(HttpResponse::InternalServerError().body("Could not find playlist"))
        }
    }
}