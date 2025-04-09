use actix_files::NamedFile;
use std::path::PathBuf;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use regex::Regex;
use actix_multipart::form::{ MultipartForm, tempfile::TempFile, json::Json as MpJson };
use actix_web::{web, get, post, HttpResponse, HttpRequest, Responder};

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
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

#[post("/image/post")]
async fn api_save_image_file(
    MultipartForm(form): MultipartForm<UploadForm>
) -> Result<impl Responder, actix_web::Error> {
    println!("POST: image");
    println!("New image: {}, size: {}", form.json.file_name, form.file.size);

    let new_file_name = format!("{}-{}", Uuid::new_v4(), sanitize_path(&form.json.file_name));
    let path = format!("./images/{}", &new_file_name);

    match form.file.file.persist(&path) {
        Ok(_) => println!("File saved at {}", &path),
        Err(e) => println!("Error: {}", e),
    };

    let resp = Metadata{
        file_name: new_file_name,
    };

    Ok(HttpResponse::Ok().json(resp))
}

#[get("/image/get/{file_name}")]
async fn api_get_image_file(
    req: HttpRequest,
    file_name: web::Path<String>
) -> Result<impl Responder, actix_web::Error> {
    println!("GET: image");

    let path = format!("./images/{}", sanitize_path(&file_name));
    let file_path = PathBuf::from(&path);

    if file_path.exists() {
        match NamedFile::open(file_path){
            Ok(f) => Ok(NamedFile::into_response(f, &req)),
            Err(_) => Ok(HttpResponse::NotFound().body("Could not open file"))
        }
    } else {
        Ok(HttpResponse::NotFound().body("File not found"))
    }
}
