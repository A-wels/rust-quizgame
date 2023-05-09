use actix_web::{HttpRequest, HttpResponse, Result};
use std::fs;

// load static files from /static/index.html

pub async fn quiz(_req: HttpRequest) -> Result<HttpResponse> {
    let contents = fs::read_to_string("./src/frontend/static/quiz.html")?;
    Ok(HttpResponse::Ok().body(contents))
}