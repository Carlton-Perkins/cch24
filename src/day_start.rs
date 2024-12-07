use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn day_start() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
pub async fn seek() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "https://www.youtube.com/watch?v=9Gc4QTqslN4"))
        .finish()
}
