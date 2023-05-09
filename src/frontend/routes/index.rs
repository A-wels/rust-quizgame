use actix_web::{HttpRequest, HttpResponse,  Result};
use std::fs;

// load static files from /static/index.html

pub async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let contents = fs::read_to_string("./src/frontend/static/index.html")?;
    // print working directory and path to index.html
    
    Ok(HttpResponse::Ok().body(contents))
}