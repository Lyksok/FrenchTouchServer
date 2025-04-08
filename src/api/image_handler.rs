use actix_web::{post, web, HttpResponse, Responder};
use actix_multipart::form::{ MultipartForm, tempfile::TempFile, json::Json as MpJson };
//use std::sync::Mutex;
use std::fs;
use uuid::Uuid;
use serde::Deserialize;
use regex::Regex;

use crate::api::run_api::AppState;

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "10MB")]
    file: TempFile,
    json: MpJson<Metadata>,
}

fn sanitize_path(s: &str) -> String {
    let re = Regex::new(r"(\.\.)|[^a-zA-Z0-9_\-\.]").unwrap();
    re.replace_all(s, "").to_string()
}

#[post("/image")]
pub async fn save_image_file(
    data: web::Data<AppState>,
    MultipartForm(form): MultipartForm<UploadForm>
) -> Result<impl Responder, actix_web::Error> {
    let _conn = data.db.lock().unwrap();
    println!("New image: {}, size: {}", form.json.name, form.file.size);

    let temp_path = form.file.file.path().to_path_buf();
    let path = format!("./images/{}-{}", Uuid::new_v4(), sanitize_path(&form.json.name));

    match fs::copy(&temp_path, &path) {
        Ok(_) => println!("File saved at {}", &path),
        Err(e) => println!("Error: {}", e),
    };

    Ok(HttpResponse::Ok())
}
