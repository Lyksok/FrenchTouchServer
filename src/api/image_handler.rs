use actix_web::{post, web, HttpResponse, Responder};
use actix_multipart::form::{ MultipartForm, tempfile::TempFile, json::Json as MpJson };
//use std::sync::Mutex;
use std::fs::File;
use serde::Deserialize;

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

#[post("/image/{path}")]
pub async fn save_image_file(
    data: web::Data<AppState>,
    MultipartForm(form): MultipartForm<UploadForm>
) -> Result<impl Responder, actix_web::Error> {
    let _conn = data.db.lock().unwrap();
    println!("New image: {}, size: {}", form.json.name, form.file.size);

    let _saved_file: File = form.file.file.into_file();
    //let dir: &str = "./images/";

    Ok(HttpResponse::Ok())
}
