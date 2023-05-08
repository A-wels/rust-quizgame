use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use std::fs;

// load static files from /static/index.html

pub async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let contents = fs::read_to_string("./static/index.html")?;
    Ok(HttpResponse::Ok().body(contents))
}