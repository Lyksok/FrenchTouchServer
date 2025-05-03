use actix_web::{HttpResponse, Responder, post, web};

use crate::api::api_utils::{
    print_log, AlbumRequest, ArtistObjRequest, CollaboratorObjRequest, CollaboratorRequestRequest, HistoryRequest, PlaylistRequest, SongAlbumRequest, SongPlaylistRequest, SongRequest, UserLikesAlbumRequest, UserLikesPlaylistRequest, UserLikesSongRequest
};
use crate::api::run_api::AppState;
use crate::db;
use crate::db::db_security::{
    has_exact_permissions, has_permissions, has_permissions_user_history,
    has_permissions_user_likes_album, has_permissions_user_likes_playlist,
    has_permissions_user_likes_song,
};
use crate::db::structs::{RequestToAdmin, RequestToArtist, RequestToCollaborator, User};

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
            match db::db_update::update_authmap(&conn, id, 1){
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
            match db::db_update::update_authmap(&conn, id, 2){
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

#[post("/insert/user-likes-song")]
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

#[post("/insert/user-likes-album")]
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

#[post("/insert/user-likes-playlist")]
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

#[post("/insert/song-album")]
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

#[post("/insert/song-playlist")]
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
    let collab_req = collab_req.obj.clone();

    match db::db_insert::insert_collaboration_request(&conn, &collab_req) {
        Some(_) => {
            println!("[INSERT] History {:?}", collab_req);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!("[ERROR] Could not insert history {:?}", collab_req);
            Ok(HttpResponse::InternalServerError().body("Could not insert history"))
        }
    }
}

#[post("/insert/request_to_artist")]
async fn api_insert_request_to_artist(
    data: web::Data<AppState>,
    req: web::Json<RequestToArtist>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    match db::db_insert::insert_request_to_artist(&conn, &req) {
        Some(_) => {
            println!("[INSERT] RequestToArtist {:?}", req);
            Ok(HttpResponse::Ok().body(""))
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
    req: web::Json<RequestToCollaborator>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    match db::db_insert::insert_request_to_collaborator(&conn, &req) {
        Some(_) => {
            println!("[INSERT] RequestToCollaborator {:?}", req);
            Ok(HttpResponse::Ok().body(""))
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
    req: web::Json<RequestToAdmin>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    match db::db_insert::insert_request_to_admin(&conn, &req) {
        Some(_) => {
            println!("[INSERT] RequestToAdmin {:?}", req);
            Ok(HttpResponse::Ok().body(""))
        }
        None => {
            println!("[ERROR] Could not insert RequestToAdmin {:?}", req);
            Ok(HttpResponse::InternalServerError().body("Could not insert RequestToAdmin"))
        }
    }
}