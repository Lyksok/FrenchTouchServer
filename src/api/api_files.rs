use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use futures::StreamExt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    file_name: String,
}

const MAX_FILE_SIZE: usize = 20 * 1024 * 1024; // 20MB

fn sanitize_path(s: &str) -> String {
    let re = Regex::new(r"(\.\.)|[^a-zA-Z0-9_\-\.]").unwrap();
    re.replace_all(s, "").to_string()
}

#[post("/post/image")]
async fn api_save_image_file(
    mut payload: web::Payload,
) -> Result<impl Responder, actix_web::Error> {
    println!("POST: image");

    // Define the path where the file will be saved
    let mut name = Uuid::new_v4();
    let mut path: String = format!("./data/images/{}.png", &name);
    while Path::new(&path).try_exists().unwrap() {
        name = Uuid::new_v4();
        path = format!("./data/images/{}.png", &name);
    }

    // Create and open the file in write mode
    let mut file = web::block(|| File::create(path))
        .await?
        .map_err(actix_web::Error::from)?;

    let mut total_size = 0;

    // Write the incoming bytes to the file
    while let Some(chunk) = payload.next().await {
        let data = chunk?;
        total_size += data.len();

        if total_size > MAX_FILE_SIZE {
            return Ok(HttpResponse::PayloadTooLarge()
                .body("File size exceeds the maximum allowed size of 20MB"));
        }

        // Write the chunk to the file
        file.write_all(&data)?;
    }

    Ok(HttpResponse::Ok().body(format!("{}.png", name)))
}

#[get("/get/image/{file_name}")]
async fn api_get_image_file(
    req: HttpRequest,
    file_name: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    println!("GET: image");

    let path = format!("./data/images/{}", sanitize_path(&file_name));
    let file_path = PathBuf::from(&path);

    if file_path.exists() {
        match NamedFile::open(file_path) {
            Ok(f) => Ok(NamedFile::into_response(f, &req)),
            Err(_) => Ok(HttpResponse::NotFound().body("Could not open file")),
        }
    } else {
        Ok(HttpResponse::NotFound().body("File not found"))
    }
}

#[post("/post/song")]
async fn api_save_song_file(mut payload: web::Payload) -> Result<impl Responder, actix_web::Error> {
    println!("POST: image");

    // Define the path where the file will be saved
    let mut name = Uuid::new_v4();
    let mut path: String = format!("./data/songs/{}.mp3", &name);
    while Path::new(&path).try_exists().unwrap() {
        name = Uuid::new_v4();
        path = format!("./data/songs/{}.mp3", &name);
    }

    // Create and open the file in write mode
    let mut file = web::block(|| File::create(path))
        .await?
        .map_err(actix_web::Error::from)?;

    let mut total_size = 0;

    // Write the incoming bytes to the file
    while let Some(chunk) = payload.next().await {
        let data = chunk?;
        total_size += data.len();

        if total_size > MAX_FILE_SIZE {
            return Ok(HttpResponse::PayloadTooLarge()
                .body("File size exceeds the maximum allowed size of 20MB"));
        }

        // Write the chunk to the file
        file.write_all(&data)?;
    }

    Ok(HttpResponse::Ok().body(format!("{}.mp3", name)))
}

#[get("/get/song/{file_name}")]
async fn api_get_song_file(
    req: HttpRequest,
    file_name: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    println!("GET: song");

    let path = format!("./data/songs/{}", sanitize_path(&file_name));
    let file_path = PathBuf::from(&path);

    if file_path.exists() {
        match NamedFile::open(file_path) {
            Ok(f) => Ok(NamedFile::into_response(f, &req)),
            Err(_) => Ok(HttpResponse::NotFound().body("Could not open file")),
        }
    } else {
        Ok(HttpResponse::NotFound().body("File not found"))
    }
}
