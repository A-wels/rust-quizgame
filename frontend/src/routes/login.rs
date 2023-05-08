use actix_web::{ HttpRequest, HttpResponse, Result};
use std::fs;

// load static files from /static/index.html

pub async fn login(_req: HttpRequest) -> Result<HttpResponse> {
    let contents = fs::read_to_string("./static/login.html")?;
    Ok(HttpResponse::Ok().body(contents))
}