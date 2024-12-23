use actix_web::{get, HttpResponse, Responder};

#[get("/23/star")]
pub async fn star() -> impl Responder {
    HttpResponse::Ok().body("<div class=\"lit\" id=\"star\"></div>")
}
