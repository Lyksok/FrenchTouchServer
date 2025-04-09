use uuid::Uuid;
use serde::{Deserialize, Serialize};
use regex::Regex;
use actix_multipart::form::{ MultipartForm, tempfile::TempFile, json::Json as MpJson };
use std::fs;
use actix_web::{post, HttpResponse, Responder};

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
async fn api_save_image_file(
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
