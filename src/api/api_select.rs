use crate::api::run_api::AppState;
use crate::db;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/select/admin/email/{email}")]
async fn api_select_admin_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_admin_by_email(&conn, &email) {
        Some(admin) => Ok(HttpResponse::Ok().json(admin)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the admin")),
    }
}

#[get("/select/user/email/{email}")]
async fn api_select_user_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_email(&conn, &email) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the user")),
    }
}

#[get("/select/user/id/{id}")]
async fn api_select_user_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_user_by_id(&conn, id) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the user")),
    }
}

#[get("/select/user/username/{username}")]
async fn api_select_user_by_username(
    data: web::Data<AppState>,
    username: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_username(&conn, &username) {
        Some(users) => Ok(HttpResponse::Ok().json(users)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the user")),
    }
}

#[get("/select/artist/email/{email}")]
async fn api_select_artist_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_artist_by_email(&conn, &email) {
        Some(artist) => Ok(HttpResponse::Ok().json(artist)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the artist")),
    }
}

#[get("/select/artist/id/{id}")]
async fn api_select_artist_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_artist_by_id(&conn, id) {
        Some(artist) => Ok(HttpResponse::Ok().json(artist)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the artist")),
    }
}

#[get("/select/artist/username/{username}")]
async fn api_select_artist_by_username(
    data: web::Data<AppState>,
    username: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_artist_by_username(&conn, &username) {
        Some(artists) => Ok(HttpResponse::Ok().json(artists)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the artist")),
    }
}

#[get("/select/artist/user_id/{user_id}")]
async fn api_select_artist_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_artist_by_user_id(&conn, id) {
        Some(artists) => Ok(HttpResponse::Ok().json(artists)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find the artist")),
    }
}

#[get("/select/collaborator/email/{email}")]
async fn api_select_collaborator_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_collaborator_by_email(&conn, &email) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find collaborator")),
    }
}

#[get("/select/collaborator/id/{id}")]
async fn api_select_collaborator_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_collaborator_by_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find collaborator")),
    }
}
#[get("/select/song/id/{id}")]
async fn api_select_song_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_song_by_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find song")),
    }
}
#[get("/select/song/title/{title}")]
async fn api_select_song_by_title(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_song_by_title(&conn, &title) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find song")),
    }
}
#[get("/select/song/artist_id/{artist_id}")]
async fn api_select_song_by_artist_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_song_by_artist_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find song")),
    }
}

#[get("/select/album/id/{id}")]
async fn api_select_album_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_album_by_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find album")),
    }
}
#[get("/select/album/title/{title}")]
async fn api_select_album_by_title(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_album_by_title(&conn, &title) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find album")),
    }
}
#[get("/select/album/artist_id/{artist_id}")]
async fn api_select_album_by_artist_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_album_by_artist_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find album")),
    }
}
#[get("/select/playlist/id/{id}")]
async fn api_select_playlist_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_playlist_by_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find playlist")),
    }
}
#[get("/select/playlist/title/{title}")]
async fn api_select_playlist_by_title(
    data: web::Data<AppState>,
    title: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_playlist_by_title(&conn, &title) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find playlist")),
    }
}
#[get("/select/playlist/user_id/{user_id}")]
async fn api_select_playlist_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_playlist_by_user_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find playlist")),
    }
}

#[get("/select/user_likes_song/user_id/{user_id}")]
async fn api_select_user_likes_song_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_user_likes_song_by_user_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find user likes song")),
    }
}

#[get("/select/user_likes_song/song_id/{song_id}")]
async fn api_select_user_likes_song_by_song_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_user_likes_song_by_song_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find user likes song")),
    }
}
#[get("/select/user_likes_album/user_id/{user_id}")]
async fn api_select_user_likes_album_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_user_likes_album_by_user_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find user likes album")),
    }
}
#[get("/select/user_likes_album/album_id/{album_id}")]
async fn api_select_user_likes_album_by_album_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_user_likes_album_by_album_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find user likes album")),
    }
}
#[get("/select/user_likes_playlist/user_id/{user_id}")]
async fn api_select_user_likes_playlist_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_user_likes_playlist_by_user_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find user likes playlist")),
    }
}
#[get("/select/user_likes_playlist/playlist_id/{playlist_id}")]
async fn api_select_user_likes_playlist_by_playlist_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_user_likes_playlist_by_playlist_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find user likes playlist")),
    }
}

#[get("/select/song_album/song_id/{song_id}")]
async fn api_select_song_album_by_song_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_song_album_by_song_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find song album")),
    }
}
#[get("/select/song_album/album_id/{album_id}")]
async fn api_select_song_album_by_album_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_song_album_by_album_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find song album")),
    }
}
#[get("/select/song_playlist/song_id/{song_id}")]
async fn api_select_song_playlist_by_song_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_song_playlist_by_song_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find song playlist")),
    }
}
#[get("/select/song_playlist/playlist_id/{playlist_id}")]
async fn api_select_song_playlist_by_playlist_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_song_playlist_by_playlist_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find song playlist")),
    }
}
#[get("/select/history/user_id/{user_id}")]
async fn api_select_history_by_user_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_history_by_user_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find history")),
    }
}
#[get("/select/history/song_id/{song_id}")]
async fn api_select_history_by_song_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    let id = match id.parse::<i64>() {
        Err(_) => return Ok(HttpResponse::BadRequest().body("You did not provide a correct id")),
        Ok(id) => id,
    };
    match db::db_select::select_history_by_song_id(&conn, id) {
        Some(e) => Ok(HttpResponse::Ok().json(e)),
        None => Ok(HttpResponse::InternalServerError().body("Could not find history")),
    }
}
