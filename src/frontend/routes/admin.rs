use actix_web::{HttpRequest, HttpResponse, Result};
use std::fs;

// load static files from /static/index.html

pub async fn admin(_req: HttpRequest) -> Result<HttpResponse> {
    let contents = fs::read_to_string("./src/frontend/static/admin.html")?;
    Ok(HttpResponse::Ok().body(contents))
}
