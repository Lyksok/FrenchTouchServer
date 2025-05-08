use actix_web::{post, web, HttpResponse, Responder};

use crate::{api::api_utils::{print_log, AuthHash, UserLikesAlbumRequest, UserLikesPlaylistRequest, UserLikesSongRequest}, db::{self, db_security::{has_permissions, has_permissions_user_likes_album, has_permissions_user_likes_playlist, has_permissions_user_likes_song}}};

use super::run_api::AppState;

#[post("/delete/collaborator_request/id/{id}")]
async fn api_delete_collaborator_request_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR DELETE", "User permission (CollaboratorRequest)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_collaborator_request_by_id(&conn, *id) {
        true => {
            print_log("DELETE", "CollaboratorRequest", &id);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "CollaboratorRequest", &id);
            return Ok(HttpResponse::InternalServerError().body("Could not delete CollaboratorRequest"))
        },
    }
}

#[post("/delete/artist_request/id/{id}")]
async fn api_delete_artist_request_by_id(
    data: web::Data<AppState>,
    id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR DELETE", "User permission (ArtistRequest)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_artist_request_by_id(&conn, *id) {
        true => {
            print_log("DELETE", "ArtistRequest", &id);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "ArtistRequest", &id);
            return Ok(HttpResponse::InternalServerError().body("Could not delete ArtistRequest"))
        },
    }
}

#[post("/delete/request_to_collaborator/user_id/{user_id}")]
async fn api_delete_request_to_collaborator_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR DELETE", "User permission (RequestToCollaborator)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_request_to_collaborator_by_user_id(&conn, *user_id) {
        true => {
            print_log("DELETE", "RequestToCollaborator", &user_id);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "RequestToCollaborator", &user_id);
            return Ok(HttpResponse::InternalServerError().body("Could not delete RequestToCollaborator"))
        },
    }
}

#[post("/delete/request_to_artist/user_id/{user_id}")]
async fn api_delete_request_to_artist_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR DELETE", "User permission (RequestToArtist)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_request_to_artist_by_user_id(&conn, *user_id) {
        true => {
            print_log("DELETE", "RequestToArtist", &user_id);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "RequestToArtist", &user_id);
            return Ok(HttpResponse::InternalServerError().body("Could not delete RequestToArtist"))
        },
    }
}

#[post("/delete/request_to_admin/user_id/{user_id}")]
async fn api_delete_request_to_admin_by_user_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
    auth_hash: web::Json<AuthHash>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &auth_hash.auth_hash, 3) {
        print_log("ERROR DELETE", "User permission (RequestToAdmin)", &auth_hash.auth_hash);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_request_to_admin_by_user_id(&conn, *user_id) {
        true => {
            print_log("DELETE", "RequestToAdmin", &user_id);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "RequestToAdmin", &user_id);
            return Ok(HttpResponse::InternalServerError().body("Could not delete RequestToAdmin"))
        },
    }
}

#[post("/delete/user_likes_song")]
async fn api_delete_user_likes_song(
    data: web::Data<AppState>,
    payload: web::Json<UserLikesSongRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &payload.auth_hash, 0) || !has_permissions_user_likes_song(&conn, &payload){
        print_log("ERROR DELETE", "User permission (UserLikesSong)", &payload.obj);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_user_likes_song(&conn, &payload.obj) {
        true => {
            print_log("DELETE", "UserLikesSong", &payload.obj);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "UserLikesSong", &payload.obj);
            return Ok(HttpResponse::InternalServerError().body("Could not delete UserLikesSong"))
        },
    }
}

#[post("/delete/user_likes_album")]
async fn api_delete_user_likes_album(
    data: web::Data<AppState>,
    payload: web::Json<UserLikesAlbumRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &payload.auth_hash, 0) || !has_permissions_user_likes_album(&conn, &payload){
        print_log("ERROR DELETE", "User permission (UserLikesAlbum)", &payload.obj);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_user_likes_album(&conn, &payload.obj) {
        true => {
            print_log("DELETE", "UserLikesAlbum", &payload.obj);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "UserLikesAlbum", &payload.obj);
            return Ok(HttpResponse::InternalServerError().body("Could not delete UserLikesAlbum"))
        },
    }
}

#[post("/delete/user_likes_playlist")]
async fn api_delete_user_likes_playlist(
    data: web::Data<AppState>,
    payload: web::Json<UserLikesPlaylistRequest>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();

    if !has_permissions(&conn, &payload.auth_hash, 0) || !has_permissions_user_likes_playlist(&conn, &payload){
        print_log("ERROR DELETE", "User permission (UserLikesPLaylist)", &payload.obj);
        return Ok(HttpResponse::Forbidden().body("You do not have access"));
    }
    
    match db::db_delete::delete_user_likes_playlist(&conn, &payload.obj) {
        true => {
            print_log("DELETE", "UserLikesPLaylist", &payload.obj);
            return Ok(HttpResponse::Ok().body(""))
        },
        false => {
            print_log("ERROR DELETE", "UserLikesPLaylist", &payload.obj);
            return Ok(HttpResponse::InternalServerError().body("Could not delete UserLikesPLaylist"))
        },
    }
}