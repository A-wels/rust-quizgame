use actix_web::{App, HttpServer};
use rust_embed::RustEmbed;
use actix_embed::Embed;


#[derive(RustEmbed)]
#[folder = "page/"]
struct Asset;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting local webserver");
    HttpServer::new(|| {
        App::new().service(
            Embed::new("", &Asset)
                .index_file("index.html")
        )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
