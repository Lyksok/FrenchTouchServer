use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;
use actix_multipart::form::{ MultipartForm, tempfile::TempFile, json::Json as MpJson };
//use std::sync::Mutex;
use std::fs;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use regex::Regex;

use crate::db;
use crate::db::structs::User;

pub struct AppState {
    pub db: Mutex<rusqlite::Connection>,
}

#[get("/users/select/{email}")]
async fn get_user_by_email(
    data: web::Data<AppState>,
    email: web::Path<String>
) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_email(&*conn, &email) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}",e))),
    }
}

#[post("/users/insert")]
async fn insert_user(
    data: web::Data<AppState>,
    user_data: web::Json<User>
) -> Result<impl Responder, actix_web::Error> {
    println!("/users/insert: json={:?}", &user_data);
    let conn = data.db.lock().unwrap();
    match db::db_insert::insert_user(&*conn, &user_data) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}",e))),
    }
}

#[post("/users/update/profile_picture/{email}")]
async fn set_user_profile_picture(
    data: web::Data<AppState>,
    user_data: web::Json<User>
) -> Result<impl Responder, actix_web::Error> {
    println!("/users/update: json={:?}", &user_data);
    let conn = data.db.lock().unwrap();
    match db::db_update::update_user_profile_picture(&*conn, &user_data) {
        Ok(()) => Ok(HttpResponse::Ok().json("")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}",e))),
    }
}

#[derive(Debug, Deserialize)]
struct Metadata {
    file_name: String,
}

#[derive(Debug, Serialize)]
struct JsonResponse {
    file_name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "10MB")]
    json: MpJson<Metadata>,
    file: TempFile,
}

fn sanitize_path(s: &str) -> String {
    let re = Regex::new(r"(\.\.)|[^a-zA-Z0-9_\-\.]").unwrap();
    re.replace_all(s, "").to_string()
}

#[post("/image")]
pub async fn save_image_file(
    MultipartForm(form): MultipartForm<UploadForm>
) -> Result<impl Responder, actix_web::Error> {
    println!("Received new connection for image transfert");
    println!("New image: {}, size: {}", form.json.file_name, form.file.size);

    let temp_path = form.file.file.path().to_path_buf();
    let sanitized = sanitize_path(&form.json.file_name);
    let new_path = format!("{}-{}", Uuid::new_v4(), sanitized);
    let path = format!("./images/{}", &new_path);

    match fs::copy(&temp_path, &path) {
        Ok(_) => println!("File saved at {}", &path),
        Err(e) => println!("Error: {}", e),
    };

    let resp = JsonResponse{
        file_name: new_path,
    };

    Ok(HttpResponse::Ok().json(resp))
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

    let shared_state = web::Data::new(AppState { db: Mutex::new(conn) });

    println!("Server running!");
    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .service(get_user_by_email)
            .service(insert_user)
            .service(save_image_file)
            .service(set_user_profile_picture)
    })
    .bind_openssl("0.0.0.0:50000",builder)?
    .run()
    .await
}
