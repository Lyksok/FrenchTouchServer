use crate::api::api_utils::AuthHash;
use crate::api::{api_utils::print_log, run_api::AppState};
use crate::db;
use crate::db::db_security::has_permissions;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/select/admin/user_id/{user_id}")]
async fn api_select_admin_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR SELECT", "User permission (Admin by user_id)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }

    match db::db_select::select_admin_by_user_id(&conn, *user_id) {
        Some(admin) => {
            print_log("SELECT", "Admin", &admin);
            Ok(HttpResponse::Ok().json(admin))
        }
        _ => {
            print_log("ERROR SELECT", "Admin", &user_id);
            Ok(HttpResponse::InternalServerError().body("Could not find the admin"))
        }
    }
}

#[get("/select/user/email/{email}")]
async fn api_select_user_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_email(&conn, &email) {
        Some(user) => {
            print_log("SELECT", "User", &user);
            Ok(HttpResponse::Ok().json(user))
        }
        _ => {
            print_log("ERROR SELECT", "User", &email);
            println!("[ERROR] Could not find the user with email {:?}", email);
            Ok(HttpResponse::InternalServerError().body("Could not find the user"))
        }
    }
}

#[get("/select/user/id/{id}")]
async fn api_select_user_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_id(&conn, *id) {
        Some(user) => {
            print_log("SELECT", "User", &user);
            Ok(HttpResponse::Ok().json(user))
        }
        _ => {
            print_log("ERROR SELECT", "User", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find the user"))
        }
    }
}

#[get("/select/user/username/{username}")]
async fn api_select_user_by_username(
    data: web::Data<AppState>,
    username: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_username(&conn, &username) {
        Some(users) => {
            print_log("SELECT", "User", &users);
            Ok(HttpResponse::Ok().json(users))
        }
        _ => {
            print_log("ERROR SELECT", "User", &username);
            Ok(HttpResponse::InternalServerError().body("Could not find the user"))
        }
    }
}

#[get("/select/artist/all")]
async fn api_select_artists(data: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_artist_all(&conn) {
        Some(artist) => {
            print_log("SELECT", "Artist", &artist);
            Ok(HttpResponse::Ok().json(artist))
        }
        _ => {
            print_log("ERROR SELECT", "Artist", &"all");
            Ok(HttpResponse::InternalServerError().body("Could not find the artist"))
        }
    }
}

#[get("/select/artist/email/{email}")]
async fn api_select_artist_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_artist_by_email(&conn, &email) {
        Some(artist) => {
            print_log("SELECT", "Artist", &artist);
            Ok(HttpResponse::Ok().json(artist))
        }
        _ => {
            print_log("ERROR SELECT", "Artist", &email);
            Ok(HttpResponse::InternalServerError().body("Could not find the artist"))
        }
    }
}

#[get("/select/artist/id/{id}")]
async fn api_select_artist_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_artist_by_id(&conn, *id) {
        Some(artist) => {
            print_log("SELECT", "Artist", &artist);
            Ok(HttpResponse::Ok().json(artist))
        }
        _ => {
            print_log("ERROR SELECT", "Artist", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find the artist"))
        }
    }
}

#[get("/select/artist/username/{username}")]
async fn api_select_artist_by_username(
    data: web::Data<AppState>,
    username: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_artist_by_username(&conn, &username) {
        Some(artists) => {
            print_log("SELECT", "Artist", &artists);
            Ok(HttpResponse::Ok().json(artists))
        }
        _ => {
            print_log("ERROR SELECT", "Artist", &username);
            Ok(HttpResponse::InternalServerError().body("Could not find the artist"))
        }
    }
}

#[get("/select/artist/user_id/{user_id}")]
async fn api_select_artist_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_artist_by_user_id(&conn, *id) {
        Some(artists) => {
            print_log("SELECT", "Artist", &artists);
            Ok(HttpResponse::Ok().json(artists))
        }
        _ => {
            print_log("ERROR SELECT", "Artist", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find the artist"))
        }
    }
}

#[get("/select/collaborator/email/{email}")]
async fn api_select_collaborator_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_collaborator_by_email(&conn, &email) {
        Some(collaborator) => {
            print_log("SELECT", "Collaborator", &collaborator);
            Ok(HttpResponse::Ok().json(collaborator))
        }
        _ => {
            print_log("ERROR SELECT", "Collaborator", &email);
            Ok(HttpResponse::InternalServerError().body("Could not find collaborator"))
        }
    }
}

#[get("/select/collaborator/id/{id}")]
async fn api_select_collaborator_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_collaborator_by_id(&conn, *id) {
        Some(collaborator) => {
            print_log("SELECT", "Collaborator", &collaborator);
            Ok(HttpResponse::Ok().json(collaborator))
        }
        _ => {
            print_log("ERROR SELECT", "Collaborator", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find collaborator"))
        }
    }
}

#[get("/select/collaborator/user_id/{user_id}")]
async fn api_select_collaborator_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_collaborator_by_user_id(&conn, *user_id) {
        Some(collaborator) => {
            print_log("SELECT", "Collaborator", &collaborator);
            Ok(HttpResponse::Ok().json(collaborator))
        }
        _ => {
            print_log("ERROR SELECT", "Collaborator", &user_id);
            Ok(HttpResponse::InternalServerError().body("Could not find collaborator"))
        }
    }
}
#[get("/select/song/id/{id}")]
async fn api_select_song_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_song_by_id(&conn, *id) {
        Some(song) => {
            print_log("SELECT", "Song", &song);
            Ok(HttpResponse::Ok().json(song))
        }
        _ => {
            print_log("ERROR SELECT", "Song", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find song"))
        }
    }
}

#[get("/select/song/title/{title}")]
async fn api_select_song_by_title(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_song_by_title(&conn, &title) {
        Some(song) => {
            print_log("SELECT", "Song", &song);
            Ok(HttpResponse::Ok().json(song))
        }
        _ => {
            print_log("ERROR SELECT", "Song", &title);
            Ok(HttpResponse::InternalServerError().body("Could not find song"))
        }
    }
}

#[get("/select/song/artist_id/{artist_id}")]
async fn api_select_song_by_artist_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_song_by_artist_id(&conn, *id) {
        Some(song) => {
            print_log("SELECT", "Song", &song);
            Ok(HttpResponse::Ok().json(song))
        }
        _ => {
            print_log("ERROR SELECT", "Song", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find song"))
        }
    }
}

#[get("/select/album/id/{id}")]
async fn api_select_album_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_album_by_id(&conn, *id) {
        Some(album) => {
            print_log("SELECT", "Album", &album);
            Ok(HttpResponse::Ok().json(album))
        }
        _ => {
            print_log("ERROR SELECT", "Album", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find album"))
        }
    }
}

#[get("/select/album/title/{title}")]
async fn api_select_album_by_title(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_album_by_title(&conn, &title) {
        Some(album) => {
            print_log("SELECT", "Album", &album);
            Ok(HttpResponse::Ok().json(album))
        }
        _ => {
            print_log("ERROR SELECT", "Album", &title);
            Ok(HttpResponse::InternalServerError().body("Could not find album"))
        }
    }
}

#[get("/select/album/artist_id/{artist_id}")]
async fn api_select_album_by_artist_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_album_by_artist_id(&conn, *id) {
        Some(album) => {
            print_log("SELECT", "Album", &album);
            Ok(HttpResponse::Ok().json(album))
        }
        _ => {
            print_log("ERROR SELECT", "Album", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find album"))
        }
    }
}

#[get("/select/playlist/id/{id}")]
async fn api_select_playlist_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_playlist_by_id(&conn, *id) {
        Some(playlist) => {
            print_log("SELECT", "Playlist", &playlist);
            Ok(HttpResponse::Ok().json(playlist))
        }
        _ => {
            print_log("ERROR SELECT", "Playlist", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find playlist"))
        }
    }
}

#[get("/select/playlist/title/{title}")]
async fn api_select_playlist_by_title(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_playlist_by_title(&conn, &title) {
        Some(playlist) => {
            print_log("SELECT", "Playlist", &playlist);
            Ok(HttpResponse::Ok().json(playlist))
        }
        _ => {
            print_log("ERROR SELECT", "Playlist", &title);
            Ok(HttpResponse::InternalServerError().body("Could not find playlist"))
        }
    }
}

#[get("/select/playlist/user_id/{user_id}")]
async fn api_select_playlist_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_playlist_by_user_id(&conn, *user_id) {
        Some(playlist) => {
            print_log("SELECT", "Playlist", &playlist);
            Ok(HttpResponse::Ok().json(playlist))
        }
        _ => {
            print_log("ERROR SELECT", "Playlist", &user_id);
            Ok(HttpResponse::InternalServerError().body("Could not find playlist"))
        }
    }
}

#[get("/select/user_likes_song/user_id/{user_id}")]
async fn api_select_user_likes_song_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_likes_song_by_user_id(&conn, *user_id) {
        Some(user_likes_song) => {
            print_log("SELECT", "UserLikesSong", &user_likes_song);
            Ok(HttpResponse::Ok().json(user_likes_song))
        }
        _ => {
            print_log("ERROR SELECT", "UserLikesSong", &user_id);
            Ok(HttpResponse::InternalServerError().body("Could not find user likes song"))
        }
    }
}

#[get("/select/user_likes_song/song_id/{song_id}")]
async fn api_select_user_likes_song_by_song_id(
    data: web::Data<AppState>,
    song_id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_likes_song_by_song_id(&conn, *song_id) {
        Some(user_likes_song) => {
            print_log("SELECT", "UserLikesSong", &user_likes_song);
            Ok(HttpResponse::Ok().json(user_likes_song))
        }
        _ => {
            print_log("ERROR SELECT", "UserLikesSong", &song_id);
            Ok(HttpResponse::InternalServerError().body("Could not find user likes song"))
        }
    }
}

#[get("/select/user_likes_album/user_id/{user_id}")]
async fn api_select_user_likes_album_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_likes_album_by_user_id(&conn, *id) {
        Some(user_likes_album) => {
            print_log("SELECT", "UserLikesAlbum", &user_likes_album);
            Ok(HttpResponse::Ok().json(user_likes_album))
        }
        _ => {
            print_log("ERROR SELECT", "UserLikesAlbum", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find user likes album"))
        }
    }
}

#[get("/select/user_likes_album/album_id/{album_id}")]
async fn api_select_user_likes_album_by_album_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_likes_album_by_album_id(&conn, *id) {
        Some(user_likes_album) => {
            print_log("SELECT", "UserLikesAlbum", &user_likes_album);
            Ok(HttpResponse::Ok().json(user_likes_album))
        }
        _ => {
            print_log("ERROR SELECT", "UserLikesAlbum", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find user likes album"))
        }
    }
}

#[get("/select/user_likes_playlist/user_id/{user_id}")]
async fn api_select_user_likes_playlist_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_likes_playlist_by_user_id(&conn, *id) {
        Some(user_likes_playlist) => {
            print_log("SELECT", "UserLikesPlaylist", &user_likes_playlist);
            Ok(HttpResponse::Ok().json(user_likes_playlist))
        }
        _ => {
            print_log("ERROR SELECT", "UserLikesAlbum", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find user likes playlist"))
        }
    }
}

#[get("/select/user_likes_playlist/playlist_id/{playlist_id}")]
async fn api_select_user_likes_playlist_by_playlist_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_likes_playlist_by_playlist_id(&conn, *id) {
        Some(user_likes_playlist) => {
            print_log("SELECT", "UserLikesPlaylist", &user_likes_playlist);
            Ok(HttpResponse::Ok().json(user_likes_playlist))
        }
        _ => {
            print_log("ERROR SELECT", "UserLikesAlbum", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find user likes playlist"))
        }
    }
}

#[get("/select/song_album/song_id/{song_id}")]
async fn api_select_song_album_by_song_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_song_album_by_song_id(&conn, *id) {
        Some(song_album) => {
            print_log("SELECT", "SongAlbum", &song_album);
            Ok(HttpResponse::Ok().json(song_album))
        }
        _ => {
            print_log("ERROR SELECT", "SongAlbum", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find song album"))
        }
    }
}

#[get("/select/song_album/album_id/{album_id}")]
async fn api_select_song_album_by_album_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_song_album_by_album_id(&conn, *id) {
        Some(song_album) => {
            print_log("SELECT", "SongAlbum", &song_album);
            Ok(HttpResponse::Ok().json(song_album))
        }
        _ => {
            print_log("ERROR SELECT", "SongAlbum", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find song album"))
        }
    }
}

#[get("/select/song_playlist/song_id/{song_id}")]
async fn api_select_song_playlist_by_song_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_song_playlist_by_song_id(&conn, *id) {
        Some(song_playlist) => {
            print_log("SELECT", "SongPlaylist", &song_playlist);
            Ok(HttpResponse::Ok().json(song_playlist))
        }
        _ => {
            print_log("ERROR SELECT", "SongPlaylist", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find song playlist"))
        }
    }
}

#[get("/select/song_playlist/playlist_id/{playlist_id}")]
async fn api_select_song_playlist_by_playlist_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_song_playlist_by_playlist_id(&conn, *id) {
        Some(song_playlist) => {
            print_log("SELECT", "SongPlaylist", &song_playlist);
            Ok(HttpResponse::Ok().json(song_playlist))
        }
        _ => {
            print_log("ERROR SELECT", "SongPlaylist", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find song playlist"))
        }
    }
}

#[get("/select/history/user_id/{user_id}")]
async fn api_select_history_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_history_by_user_id(&conn, *id) {
        Some(history) => {
            print_log("SELECT", "History", &history);
            Ok(HttpResponse::Ok().json(history))
        }
        _ => {
            print_log("ERROR SELECT", "History", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find history"))
        }
    }
}

#[get("/select/history/song_id/{song_id}")]
async fn api_select_history_by_song_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_history_by_song_id(&conn, *id) {
        Some(history) => {
            print_log("SELECT", "History", &history);
            Ok(HttpResponse::Ok().json(history))
        }
        _ => {
            print_log("ERROR SELECT", "History", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find history"))
        }
    }
}

#[get("/select/artist_request/id/{id}")]
async fn api_select_artist_request_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    match db::db_select::select_artist_request_by_id(&conn, *id) {
        Some(artist_req) => {
            print_log("SELECT", "ArtistRequest", &artist_req);
            Ok(HttpResponse::Ok().json(artist_req))
        }
        _ => {
            print_log("ERROR SELECT", "ArtistRequest", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find the ArtistRequest"))
        }
    }
}

#[get("/select/artist_request/all")]
async fn api_select_artist_request_all(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    match db::db_select::select_artist_request_all(&conn) {
        Some(artist_req) => {
            print_log("SELECT", "ArtistRequest", &artist_req);
            Ok(HttpResponse::Ok().json(artist_req))
        }
        _ => {
            print_log("ERROR SELECT", "ArtistRequest", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find the ArtistRequest"))
        }
    }
}

#[get("/select/collaborator_request/id/{id}")]
async fn api_select_collaborator_request_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    match db::db_select::select_collaborator_request_by_id(&conn, *id) {
        Some(artist_req) => {
            print_log("SELECT", "CollaboratorRequest", &artist_req);
            Ok(HttpResponse::Ok().json(artist_req))
        }
        _ => {
            print_log("ERROR SELECT", "CollaboratorRequest", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find the CollaboratorRequest"))
        }
    }
}

#[get("/select/collaborator_request/add")]
async fn api_select_collaborator_request_all(
    data: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    match db::db_select::select_collaborator_request_all(&conn) {
        Some(artist_req) => {
            print_log("SELECT", "CollaboratorRequest", &artist_req);
            Ok(HttpResponse::Ok().json(artist_req))
        }
        _ => {
            print_log("ERROR SELECT", "CollaboratorRequest", &id);
            Ok(HttpResponse::InternalServerError().body("Could not find the CollaboratorRequest"))
        }
    }
}

#[get("/select/request_to_artist/user_id/{user_id}")]
async fn api_select_request_to_artist_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_request_to_artist_by_user_id(&conn, *user_id) {
        Some(elt) => {
            print_log("SELECT", "RequestToArtist", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        _ => {
            print_log("ERROR SELECT", "RequestToArtist", &"all");
            Ok(HttpResponse::InternalServerError().body("Could not find RequestToArtist"))
        }
    }
}

#[get("/select/request_to_collaborator/user_id/{user_id}")]
async fn api_select_request_to_collaborator_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_request_to_collaborator_by_user_id(&conn, *user_id) {
        Some(elt) => {
            print_log("SELECT", "RequestToCollaborator", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        _ => {
            print_log("ERROR SELECT", "RequestToCollaborator", &"all");
            Ok(HttpResponse::InternalServerError().body("Could not find RequestToCollaborator"))
        }
    }
}

#[get("/select/request_to_admin/user_id/{user_id}")]
async fn api_select_request_to_admin_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_request_to_admin_by_user_id(&conn, *user_id) {
        Some(elt) => {
            print_log("SELECT", "RequestToAdmin", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        _ => {
            print_log("ERROR SELECT", "RequestToAdmin", &"all");
            Ok(HttpResponse::InternalServerError().body("Could not find RequestToAdmin"))
        }
    }
}

#[get("/select/request_to_artist/all")]
async fn api_select_request_to_artist_all(
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_request_to_artist_all(&conn) {
        Some(elt) => {
            print_log("SELECT", "RequestToArtist", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        _ => {
            print_log("ERROR SELECT", "RequestToArtist", &"all");
            Ok(HttpResponse::InternalServerError().body("Could not find RequestToArtist"))
        }
    }
}

#[get("/select/request_to_collaborator/all")]
async fn api_select_request_to_collaborator_all(
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_request_to_collaborator_all(&conn) {
        Some(elt) => {
            print_log("SELECT", "RequestToCollaborator", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        _ => {
            print_log("ERROR SELECT", "RequestToCollaborator", &"all");
            Ok(HttpResponse::InternalServerError().body("Could not find RequestToCollaborator"))
        }
    }
}

#[get("/select/request_to_admin/all")]
async fn api_select_request_to_admin_all(
    data: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_request_to_admin_all(&conn) {
        Some(elt) => {
            print_log("SELECT", "RequestToAdmin", &elt);
            Ok(HttpResponse::Ok().json(elt))
        }
        _ => {
            print_log("ERROR SELECT", "RequestToAdmin", &"all");
            Ok(HttpResponse::InternalServerError().body("Could not find RequestToAdmin"))
        }
    }
}