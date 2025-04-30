use actix_web::{post, web, HttpResponse, Responder};

use crate::api::api_utils::print_log;
use crate::api::run_api::AppState;
use crate::db;
use crate::db::structs::{
    Album, Artist, Collaborator, History, Playlist, Song, SongAlbum, SongPlaylist, User,
    UserLikesAlbum, UserLikesPlaylist, UserLikesSong,
};

#[post("/insert/user")]
async fn api_insert_user(
    data: web::Data<AppState>,
    mut user_data: web::Json<User>,
) -> Result<impl Responder, actix_web::Error> {
    println!("/users/insert: json={:?}", &user_data);
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_user(&conn, &user_data) {
        Some(id) => {
            user_data.id = id;
            print_log("INSERT", "User", &user_data);
            Ok(HttpResponse::Ok().json(user_data))
        }
        _ => {
            print_log("ERROR INSERT", "User", &user_data);
            Ok(HttpResponse::InternalServerError().body("Could not insert user."))
        }
    }
}

#[post("/insert/artist")]
async fn api_insert_artist(
    data: web::Data<AppState>,
    mut artist_data: web::Json<Artist>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_artist(&conn, &artist_data) {
        Some(id) => {
            artist_data.id = id;
            print_log("INSERT", "Artist", &artist_data);
            Ok(HttpResponse::Ok().json(artist_data))
        }
        None => {
            print_log("ERROR INSERT", "Artist", &artist_data);
            Ok(HttpResponse::InternalServerError().body("Could not insert artist."))
        }
    }
}

#[post("/insert/collaborator")]
async fn api_insert_collaborator(
    data: web::Data<AppState>,
    mut collaborator_data: web::Json<Collaborator>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_collaborator(&conn, &collaborator_data) {
        Some(id) => {
            collaborator_data.id = id;
            println!("[INSERT] Collaborator {:?}", collaborator_data);
            Ok(HttpResponse::Ok().json(collaborator_data))
        }
        None => {
            println!(
                "[ERROR] Could not insert collaborator {:?}",
                collaborator_data
            );
            Ok(HttpResponse::InternalServerError().body("Could not insert collaborator."))
        }
    }
}

#[post("/insert/song")]
async fn api_insert_song(
    data: web::Data<AppState>,
    mut song_data: web::Json<Song>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_song(&conn, &song_data) {
        Some(id) => {
            song_data.id = id;
            println!("[INSERT] Song {:?}", song_data);
            Ok(HttpResponse::Ok().json(song_data))
        }
        None => {
            println!("[ERROR] Could not insert song {:?}", song_data);
            Ok(HttpResponse::InternalServerError().body("Could not insert song"))
        }
    }
}

#[post("/insert/album")]
async fn api_insert_album(
    data: web::Data<AppState>,
    mut album_data: web::Json<Album>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_album(&conn, &album_data) {
        Some(id) => {
            album_data.id = id;
            println!("[INSERT] Album {:?}", album_data);
            Ok(HttpResponse::Ok().json(album_data))
        }
        None => {
            println!("[ERROR] Could not insert album {:?}", album_data);
            Ok(HttpResponse::InternalServerError().body("Could not insert album"))
        }
    }
}

#[post("/insert/playlist")]
async fn api_insert_playlist(
    data: web::Data<AppState>,
    mut playlist_data: web::Json<Playlist>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_playlist(&conn, &playlist_data) {
        Some(id) => {
            playlist_data.id = id;
            println!("[INSERT] Playlist {:?}", playlist_data);
            Ok(HttpResponse::Ok().json(playlist_data))
        }
        None => {
            println!("[ERROR] Could not insert playlist {:?}", playlist_data);
            Ok(HttpResponse::InternalServerError().body("Could not insert playlist"))
        }
    }
}

#[post("/insert/user-likes-song")]
async fn api_insert_user_likes_song(
    data: web::Data<AppState>,
    uls_data: web::Json<UserLikesSong>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_user_likes_song(&conn, &uls_data) {
        Some(_) => {
            println!("[INSERT] User likes song {:?}", uls_data);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!("[ERROR] Could not insert user likes song {:?}", uls_data);
            Ok(HttpResponse::InternalServerError().body("Could not insert user likes song"))
        }
    }
}

#[post("/insert/user-likes-album")]
async fn api_insert_user_likes_album(
    data: web::Data<AppState>,
    ula_data: web::Json<UserLikesAlbum>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_user_likes_album(&conn, &ula_data) {
        Some(_) => {
            println!("[INSERT] User likes album {:?}", ula_data);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!("[ERROR] Could not insert user likes album {:?}", ula_data);
            Ok(HttpResponse::InternalServerError().body("Could not insert user likes album"))
        }
    }
}

#[post("/insert/user-likes-playlist")]
async fn api_insert_user_likes_playlist(
    data: web::Data<AppState>,
    ulp_data: web::Json<UserLikesPlaylist>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_user_likes_playlist(&conn, &ulp_data) {
        Some(_) => {
            println!("[INSERT] User likes playlist {:?}", ulp_data);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!(
                "[ERROR] Could not insert user likes playlist {:?}",
                ulp_data
            );
            Ok(HttpResponse::InternalServerError().body("Could not insert user likes playlist"))
        }
    }
}

#[post("/insert/song-album")]
async fn api_insert_song_album(
    data: web::Data<AppState>,
    song_album: web::Json<SongAlbum>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_song_album(&conn, &song_album) {
        Some(_) => {
            println!("[INSERT] User likes song album {:?}", song_album);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!(
                "[ERROR] Could not insert user likes song album {:?}",
                song_album
            );
            Ok(HttpResponse::InternalServerError().body("Could not insert user likes song album"))
        }
    }
}

#[post("/insert/song-playlist")]
async fn api_insert_song_playlist(
    data: web::Data<AppState>,
    song_playlist: web::Json<SongPlaylist>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_song_playlist(&conn, &song_playlist) {
        Some(_) => {
            println!("[INSERT] User likes song playlist {:?}", song_playlist);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!(
                "[ERROR] Could not insert user likes song playlist {:?}",
                song_playlist
            );
            Ok(HttpResponse::InternalServerError()
                .body("Could not insert user likes song playlist"))
        }
    }
}

#[post("/insert/history")]
async fn api_insert_history(
    data: web::Data<AppState>,
    history: web::Json<History>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_history(&conn, &history) {
        Some(_) => {
            println!("[INSERT] History {:?}", history);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!("[ERROR] Could not insert history {:?}", history);
            Ok(HttpResponse::InternalServerError().body("Could not insert history"))
        }
    }
}
