use actix_web::{post, web, Responder};
use actix_multipart::form::{ MultipartForm, tempfile::TempFile, json::Json as MpJson };
//use std::sync::Mutex;
use std::fs;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use regex::Regex;

use crate::api::run_api::AppState;

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, Serialize)]
struct JsonResponse {
    image_path: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "5MB")]
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
    let sanitized = sanitize_path(&form.json.name);
    let new_path = format!("{}-{}", Uuid::new_v4(), sanitized);
    let path = format!("./images/{}", new_path);

    match fs::copy(&temp_path, &path) {
        Ok(_) => println!("File saved at {}", &path),
        Err(e) => println!("Error: {}", e),
    };

    let resp = JsonResponse{
        image_path: new_path,
    };

    Ok(web::Json(resp))
}
