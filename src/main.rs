use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome! You are not using this connection as intended. \
        Contact jans.stokmanis@gmail.com for more information."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Build CA
    println!("Building ssl certificate authenticator");
    let mut builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/etc/ssl/private/server-key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("/etc/ssl/private/server-cert.pem")
        .unwrap();

    println!("Building http server");
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind_openssl("0.0.0.0:50000",builder)?
    .run()
    .await
    println!("Server running!")
}
