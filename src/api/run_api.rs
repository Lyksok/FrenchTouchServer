#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::db;
use std::sync::Mutex;

struct AppState {
    db: Mutex<rusqlite::Connection>,
}

#[get("/users/{email}")]
async fn get_users(
    data: web::Data<AppState>,
    email: web::Path<String>
    ) -> Result<impl Responder, actix_web::Error> {
    let conn = data.db.lock().unwrap();
    match db::db_select::select_user_by_email(&*conn, &email) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}",e))),
    }
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome! You are not using this connection as intended. \
        Contact jans.stokmanis@gmail.com for more information."
}

#[actix_web::main]
pub async fn run_api() -> std::io::Result<()> {
    // Build CA
    println!("Building ssl certificate authenticator");
    let mut builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/etc/ssl/private/server-key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("/etc/ssl/private/server-cert.pem")
        .unwrap();

    let conn = db::db_utils::open_db("sqlite.db").expect("Could not open database");

    let shared_state = web::Data::new(AppState { db: Mutex::new(conn) });

    println!("Server running!");
    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .service(index)
            .service(get_users)
    })
    .bind_openssl("0.0.0.0:50000",builder)?
    .run()
    .await
}
