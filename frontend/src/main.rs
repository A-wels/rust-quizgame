use actix_files::Files;
use actix_web::{web, App, HttpServer};
mod routes;
use local_ip_address::local_ip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_ip = local_ip().unwrap().to_string();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(routes::index::index))
            .service(Files::new("/assets", "./static/assets"))
            .service(web::resource("/admin").to(routes::admin::admin))
            .service(web::resource("/login").to(routes::login::login))
            .service(web::resource("/qr").to(routes::qr::qr))
            .service(web::resource("/quiz").to(routes::quiz::quiz))
    })
    .bind(format!("{}:8000", local_ip))?
    .run()
    .await
}