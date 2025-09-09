use actix_web::{get, HttpResponse, Responder};

#[get("/api/auth/ping")]
pub async fn pings()-> impl Responder {
    HttpResponse::Ok().body("pong")
}