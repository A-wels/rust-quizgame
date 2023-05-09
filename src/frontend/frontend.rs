use actix_files::Files;
use actix_web::{web, App, HttpServer};
use local_ip_address::local_ip;
use crate::frontend::routes;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let local_ip = local_ip().unwrap().to_string();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(routes::index::index))
            .service(Files::new("/assets", "./src/frontend/static/assets"))
            .service(web::resource("/admin").to(routes::admin::admin))
            .service(web::resource("/login").to(routes::login::login))
            .service(web::resource("/qr").to(routes::qr::qr))
            .service(web::resource("/quiz").to(routes::quiz::quiz))
    })
    .bind(format!("{}:8000", local_ip))?
    .run()
    .await
}