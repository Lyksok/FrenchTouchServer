use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

use crate::api::api_utils::{
    print_log, AdminRequest, AlbumRequest, ArtistObjRequest, ArtistRequestRequest, CollaboratorObjRequest, CollaboratorRequestRequest, HistoryRequest, PlaylistRequest, SongAlbumRequest, SongPlaylistRequest, SongRequest, UserLikesAlbumRequest, UserLikesArtistRequest, UserLikesPlaylistRequest, UserLikesSongRequest
};
use crate::api::run_api::AppState;
use crate::db;
use crate::db::db_security::{
    has_exact_permissions, has_permissions, has_permissions_user_history, has_permissions_user_likes_album, has_permissions_user_likes_artist, has_permissions_user_likes_playlist, has_permissions_user_likes_song
};
use crate::db::structs::{RequestToAdmin, RequestToArtist, RequestToCollaborator, User};

#[post("/insert/admin")]
async fn api_insert_admin(
    data: web::Data<AppState>,
    req: web::Json<AdminRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = req.auth_hash.clone();
    let mut req = req.obj.clone();
    if !has_permissions(&conn, &auth_hash, 3) {
        print_log("ERROR INSERT", "User permission (Admin)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }

    match db::db_insert::insert_admin(&conn, &req) {
        Some(id) => {
            req.id = id;
            print_log("INSERT", "Admin", &req);
            Ok(HttpResponse::Ok().json(req))
        }
        _ => {
            print_log("ERROR INSERT", "Admin", &req);
            Ok(HttpResponse::InternalServerError().body("Could not insert admin."))
        }
    }
}

#[post("/insert/user")]
async fn api_insert_user(
    data: web::Data<AppState>,
    mut user_data: web::Json<User>,
) -> Result<impl Responder, actix_web::Error> {
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
    artist_data: web::Json<ArtistObjRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = artist_data.auth_hash.clone();
    let mut artist_data = artist_data.obj.clone();
    if !has_permissions(&conn, &auth_hash, 3) {
        print_log("ERROR INSERT", "User permission (Artist)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }

    match db::db_insert::insert_artist(&conn, &artist_data) {
        Some(id) => {
            artist_data.id = id;
            match db::db_update::update_authmap(&conn, id, 3){
                Ok(_) => (),
                Err(_) => {print_log("ERROR INSERT", "Update permission user", &id);
                    return Ok(HttpResponse
                ::Ok().body("Could not update user permissions"))},
            }
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
    collaborator_data: web::Json<CollaboratorObjRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = collaborator_data.auth_hash.clone();
    let mut collaborator_data = collaborator_data.obj.clone();
    if !has_permissions(&conn, &auth_hash, 3) {
        print_log("ERROR INSERT", "User permission (Collaborator)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }

    match db::db_insert::insert_collaborator(&conn, &collaborator_data) {
        Some(id) => {
            collaborator_data.id = id;
            match db::db_update::update_authmap(&conn, id, 3){
                Ok(_) => (),
                Err(_) => {print_log("ERROR INSERT", "Update permission user", &id);
                    return Ok(HttpResponse
                ::Ok().body("Could not update user permissions"))},
            }
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
    song_data: web::Json<SongRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = song_data.auth_hash.clone();
    let mut song_data = song_data.obj.clone();
    if !has_permissions(&conn, &auth_hash, 3) {
        print_log("ERROR INSERT", "User permission (Song)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }

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
    album_data: web::Json<AlbumRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = album_data.auth_hash.clone();
    let mut album_data = album_data.obj.clone();
    if !has_permissions(&conn, &auth_hash, 3) {
        print_log("ERROR INSERT", "User permission (Album)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }

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
    playlist_data: web::Json<PlaylistRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = playlist_data.auth_hash.clone();
    let mut playlist_data = playlist_data.obj.clone();
    if !has_permissions(&conn, &auth_hash, 3) {
        print_log("ERROR INSERT", "User permission (Album)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }

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

#[post("/insert/user_likes_song")]
async fn api_insert_user_likes_song(
    data: web::Data<AppState>,
    uls_data: web::Json<UserLikesSongRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let auth_hash = uls_data.auth_hash.clone();
    let uls_data = uls_data.into_inner();
    if !has_permissions(&conn, &auth_hash, 0) || !has_permissions_user_likes_song(&conn, &uls_data)
    {
        print_log(
            "ERROR INSERT",
            "User permission (UserLikesSong)",
            &auth_hash,
        );
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let uls_data = uls_data.obj.clone();

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

#[post("/insert/user_likes_album")]
async fn api_insert_user_likes_album(
    data: web::Data<AppState>,
    ula_data: web::Json<UserLikesAlbumRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let ula_data = ula_data.into_inner();
    let auth_hash = ula_data.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 3) || !has_permissions_user_likes_album(&conn, &ula_data)
    {
        print_log("ERROR INSERT", "User permission (Album)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let ula_data = ula_data.obj.clone();

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

#[post("/insert/user_likes_playlist")]
async fn api_insert_user_likes_playlist(
    data: web::Data<AppState>,
    ulp_data: web::Json<UserLikesPlaylistRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let ulp_data = ulp_data.into_inner();
    let auth_hash = ulp_data.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 3)
        || !has_permissions_user_likes_playlist(&conn, &ulp_data)
    {
        print_log("ERROR INSERT", "User permission (Playlist)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let ulp_data = ulp_data.obj.clone();

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

#[post("/insert/user_likes_artist")]
async fn api_insert_user_likes_artist(
    data: web::Data<AppState>,
    ulp_data: web::Json<UserLikesArtistRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let ulp_data = ulp_data.into_inner();
    let auth_hash = ulp_data.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 3)
        || !has_permissions_user_likes_artist(&conn, &ulp_data)
    {
        print_log("ERROR INSERT", "User permission (UserLikesArtistRequest)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let ulp_data = ulp_data.obj.clone();

    match db::db_insert::insert_user_likes_artist(&conn, &ulp_data) {
        Some(_) => {
            println!("[INSERT] User likes artist {:?}", ulp_data);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!(
                "[ERROR] Could not insert user likes artist {:?}",
                ulp_data
            );
            Ok(HttpResponse::InternalServerError().body("Could not insert user likes artist"))
        }
    }
}

#[post("/insert/song_album")]
async fn api_insert_song_album(
    data: web::Data<AppState>,
    song_album: web::Json<SongAlbumRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let song_album = song_album.into_inner();
    let auth_hash = song_album.auth_hash.clone();
    if !has_exact_permissions(&conn, &auth_hash, 1) && !has_exact_permissions(&conn, &auth_hash, 3)
    {
        print_log("ERROR INSERT", "User permission (SongAlbum)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let song_album = song_album.obj.clone();

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

#[post("/insert/song_playlist")]
async fn api_insert_song_playlist(
    data: web::Data<AppState>,
    song_playlist: web::Json<SongPlaylistRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let song_playlist = song_playlist.into_inner();
    let auth_hash = song_playlist.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 2) {
        print_log("ERROR INSERT", "User permission (SongPlaylist)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let song_playlist = song_playlist.obj.clone();

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
    history: web::Json<HistoryRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let history = history.into_inner();
    let auth_hash = history.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 0) || !has_permissions_user_history(&conn, &history) {
        print_log("ERROR INSERT", "User permission (History)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let history = history.obj.clone();

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

#[post("/insert/collaborator_request")]
async fn api_insert_collaborator_request(
    data: web::Data<AppState>,
    collab_req: web::Json<CollaboratorRequestRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let collab_req = collab_req.into_inner();
    let auth_hash = collab_req.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 2) {
        print_log("ERROR INSERT", "User permission (CollaboratorRequest)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let mut collab_req = collab_req.obj.clone();

    match db::db_insert::insert_collaboration_request(&conn, &collab_req) {
        Some(id) => {
            collab_req.id = id;
            println!("[INSERT] CollaboratorRequest {:?}", collab_req);
            Ok(HttpResponse::Ok().json(collab_req))
        }
        None => {
            println!("[ERROR] Could not insert CollaboratorRequest {:?}", collab_req);
            Ok(HttpResponse::InternalServerError().body("Could not insert CollaboratorRequest"))
        }
    }
}

#[post("/insert/artist_request")]
async fn api_insert_artist_request(
    data: web::Data<AppState>,
    artist_req: web::Json<ArtistRequestRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let artist_req = artist_req.into_inner();
    let auth_hash = artist_req.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 1) {
        print_log("ERROR INSERT", "User permission (ArtistRequest)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let mut artist_req = artist_req.obj.clone();

    match db::db_insert::insert_artist_request(&conn, &artist_req) {
        Some(id) => {
            artist_req.id = id;
            println!("[INSERT] ArtistRequest {:?}", artist_req);
            Ok(HttpResponse::Ok().json(artist_req))
        }
        None => {
            println!("[ERROR] Could not insert ArtistRequest {:?}", artist_req);
            Ok(HttpResponse::InternalServerError().body("Could not insert ArtistRequest"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WrapperRequestToArtist {
    auth_hash: String,
    obj: RequestToArtist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WrapperRequestToCollaborator {
    auth_hash: String,
    obj: RequestToCollaborator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WrapperRequestToAdmin {
    auth_hash: String,
    obj: RequestToAdmin,
}

#[post("/insert/request_to_artist")]
async fn api_insert_request_to_artist(
    data: web::Data<AppState>,
    req: web::Json<WrapperRequestToArtist>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let req = req.into_inner();
    let auth_hash = req.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 0) {
        print_log("ERROR INSERT", "User permission (RequestToArtist)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let req = req.obj.clone();

    match db::db_insert::insert_request_to_artist(&conn, &req) {
        Some(_) => {
            println!("[INSERT] RequestToArtist {:?}", req);
            Ok(HttpResponse::Ok().json(req))
        }
        None => {
            println!("[ERROR] Could not insert RequestToArtist {:?}", req);
            Ok(HttpResponse::InternalServerError().body("Could not insert RequestToArtist"))
        }
    }
}

#[post("/insert/request_to_collaborator")]
async fn api_insert_request_to_collaborator(
    data: web::Data<AppState>,
    req: web::Json<WrapperRequestToCollaborator>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let req = req.into_inner();
    let auth_hash = req.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 0) {
        print_log("ERROR INSERT", "User permission (RequestToCollaborator)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let req = req.obj.clone();

    match db::db_insert::insert_request_to_collaborator(&conn, &req) {
        Some(_) => {
            println!("[INSERT] RequestToCollaborator {:?}", req);
            Ok(HttpResponse::Ok().json(req))
        }
        None => {
            println!("[ERROR] Could not insert RequestToCollaborator {:?}", req);
            Ok(HttpResponse::InternalServerError().body("Could not insert RequestToCollaborator"))
        }
    }
}

#[post("/insert/request_to_admin")]
async fn api_insert_request_to_admin(
    data: web::Data<AppState>,
    req: web::Json<WrapperRequestToAdmin>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let req = req.into_inner();
    let auth_hash = req.auth_hash.clone();
    if !has_permissions(&conn, &auth_hash, 0) {
        print_log("ERROR INSERT", "User permission (RequestToAdmin)", &auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    let req = req.obj.clone();

    match db::db_insert::insert_request_to_admin(&conn, &req) {
        Some(_) => {
            println!("[INSERT] RequestToAdmin {:?}", req);
            Ok(HttpResponse::Ok().json(req))
        }
        None => {
            println!("[ERROR] Could not insert RequestToAdmin {:?}", req);
            Ok(HttpResponse::InternalServerError().body("Could not insert RequestToAdmin"))
        }
    }
}