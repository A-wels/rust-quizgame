use actix_web::{get, HttpResponse, Responder};

#[get("/about")]
pub async fn about() -> impl Responder {
    HttpResponse::Ok().body("This is the about page!")
}