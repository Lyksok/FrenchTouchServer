use actix_web::{App, HttpServer, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;

use crate::api::api_files;
use crate::api::api_insert;
use crate::api::api_security;
use crate::api::api_select;
use crate::api::api_update;
use crate::api::api_utils::print_log;
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

    let conn = match db::db_utils::open_db("sqlite.db") {
        Ok(conn) => conn,
        Err(e) => {
            print_log("ERROR CONN", "Connection", &e.to_string());
            return Ok(());
        }
    };

    let shared_state = web::Data::new(AppState {
        db: Mutex::new(conn),
    });

    println!("Server running!");
    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .service(api_security::api_security_login)
            .service(api_security::api_security_register)
            .service(api_select::api_select_user_by_email)
            .service(api_select::api_select_user_by_id)
            .service(api_select::api_select_user_by_username)
            .service(api_select::api_select_artists)
            .service(api_select::api_select_artist_by_email)
            .service(api_select::api_select_artist_by_id)
            .service(api_select::api_select_artist_by_username)
            .service(api_select::api_select_artist_by_user_id)
            .service(api_select::api_select_collaborator_by_email)
            .service(api_select::api_select_collaborator_by_id)
            .service(api_select::api_select_collaborator_by_user_id)
            .service(api_select::api_select_song_by_id)
            .service(api_select::api_select_song_by_title)
            .service(api_select::api_select_song_by_artist_id)
            .service(api_select::api_select_album_by_id)
            .service(api_select::api_select_album_by_title)
            .service(api_select::api_select_album_by_artist_id)
            .service(api_select::api_select_playlist_by_id)
            .service(api_select::api_select_playlist_by_title)
            .service(api_select::api_select_playlist_by_user_id)
            .service(api_select::api_select_user_likes_song_by_user_id)
            .service(api_select::api_select_user_likes_song_by_song_id)
            .service(api_select::api_select_user_likes_album_by_user_id)
            .service(api_select::api_select_user_likes_album_by_album_id)
            .service(api_select::api_select_user_likes_playlist_by_user_id)
            .service(api_select::api_select_user_likes_playlist_by_playlist_id)
            .service(api_select::api_select_song_album_by_song_id)
            .service(api_select::api_select_song_album_by_album_id)
            .service(api_select::api_select_song_playlist_by_song_id)
            .service(api_select::api_select_song_playlist_by_playlist_id)
            .service(api_select::api_select_history_by_user_id)
            .service(api_select::api_select_history_by_song_id)
            .service(api_insert::api_insert_user)
            .service(api_insert::api_insert_artist)
            .service(api_insert::api_insert_collaborator)
            .service(api_insert::api_insert_album)
            .service(api_insert::api_insert_playlist)
            .service(api_files::api_save_image_file)
            .service(api_files::api_get_image_file)
            .service(api_files::api_save_song_file)
            .service(api_files::api_get_song_file)
            .service(api_update::api_update_user_profile_picture)
            .service(api_update::api_update_user_last_connection)
    })
    .bind_openssl("0.0.0.0:50000", builder)?
    .run()
    .await
}
