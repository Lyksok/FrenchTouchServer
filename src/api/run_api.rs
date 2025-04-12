use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;

use crate::api::api_files::{
    api_get_image_file, api_get_song_file, api_save_image_file, api_save_song_file,
};
use crate::api::api_insert::api_insert_user;
use crate::api::api_select::api_select_user_by_email;
use crate::api::api_update::api_update_user_profile_picture;
use crate::db;

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
}

/*
#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome! You are not using this connection as intended. \
        Contact jans.stokmanis@gmail.com for more information."
}
*/

#[actix_web::main]
pub async fn run_api() -> std::io::Result<()> {
    // Build CA
    println!("Building ssl certificate authenticator");
    let mut builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/etc/ssl/private/server.key", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("/etc/ssl/private/server.crt")
        .unwrap();

    let conn = db::db_utils::open_db("sqlite.db").expect("Could not open database");

    let shared_state = web::Data::new(AppState {
        db: Mutex::new(conn),
    });

    println!("Server running!");
    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .service(api_select_user_by_email)
            .service(api_insert_user)
            .service(api_save_image_file)
            .service(api_get_image_file)
            .service(api_save_song_file)
            .service(api_get_song_file)
            .service(api_update_user_profile_picture)
    })
    .bind_openssl("0.0.0.0:50000", builder)?
    .run()
    .await
}
