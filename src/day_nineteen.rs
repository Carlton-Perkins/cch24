use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    delete, get, post, put,
    web::{self, Data, Path},
    HttpResponse, Responder,
};
use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;

// POST /19/reset: Clear the quotes table in the database.
#[post("/19/reset")]
pub async fn reset(pool: Data<PgPool>) -> impl Responder {
    sqlx::query("TRUNCATE quotes")
        .execute(&**pool)
        .await
        .expect("Failed to truncate quotes table");

    HttpResponse::Ok().finish()
}

#[derive(sqlx::FromRow, serde::Serialize)]
struct Quote {
    id: Uuid,
    author: String,
    quote: String,
    created_at: chrono::DateTime<chrono::Utc>,
    version: i32,
}

// GET /19/cite/{id}: Respond with the quote of the given ID. Use 404 Not Found if a quote with the ID does not exist.
#[get("/19/cite/{id}")]
pub async fn cite(pool: Data<PgPool>, id: Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let Ok(quote): Result<Quote, _> = sqlx::query_as("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_one(&**pool)
        .await
    else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Ok().json(quote)
}

// DELETE /19/remove/{id}: Delete and respond with the quote of the given ID. Same 404 logic as above.
#[delete("/19/remove/{id}")]
pub async fn remove(pool: Data<PgPool>, id: Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let Ok(quote): Result<Quote, _> =
        sqlx::query_as("DELETE FROM quotes WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&**pool)
            .await
    else {
        return HttpResponse::NotFound().finish();
    };

    HttpResponse::Ok().json(quote)
}

// PUT /19/undo/{id}: Update the author and text, and increment the version number of the quote of the given ID. Respond with the updated quote. Same 404 logic as above.
#[put("/19/undo/{id}")]
pub async fn undo(
    pool: Data<PgPool>,
    id: Path<Uuid>,
    new_quote: web::Json<QuoteDraft>,
) -> impl Responder {
    let id = id.into_inner();

    let quote: Quote = match sqlx::query_as(
        "UPDATE quotes SET author = $1, quote = $2, version = version + 1 WHERE id = $3 RETURNING *",
    )
    .bind(&new_quote.author)
    .bind(&new_quote.quote)
    .bind(id)
    .fetch_one(&**pool)
    .await {
        Ok(quote) => quote,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    HttpResponse::Ok().json(quote)
}

#[derive(serde::Deserialize)]
struct QuoteDraft {
    author: String,
    quote: String,
}

// POST /19/draft: Add a quote with a random UUID v4. Respond with the quote and 201 Created.
#[post("/19/draft")]
pub async fn draft(pool: Data<PgPool>, quote: web::Json<QuoteDraft>) -> impl Responder {
    let id = Uuid::new_v4();

    let Ok(quote): Result<Quote, _> =
        sqlx::query_as("INSERT INTO quotes (id, author, quote) VALUES ($1, $2, $3) RETURNING *")
            .bind(id)
            .bind(&quote.author)
            .bind(&quote.quote)
            .fetch_one(&**pool)
            .await
    else {
        return HttpResponse::InternalServerError().finish();
    };

    HttpResponse::Created().json(quote)
}

#[derive(serde::Deserialize)]
struct Token {
    token: Option<String>,
}

#[derive(serde::Serialize)]
struct List {
    next_token: Option<String>,
    page: i32,
    quotes: Vec<Quote>,
}

#[derive(Default)]
pub struct TokenStore(Mutex<HashMap<String, i32>>);

// /19/list
#[get("/19/list")]
pub async fn list(
    pool: Data<PgPool>,
    prev_token: web::Query<Token>,
    token_store: Data<TokenStore>,
) -> impl Responder {
    let mut token_store = token_store.0.lock().unwrap();
    let offset = match &prev_token.token {
        Some(token) => token_store.remove(token),
        None => Some(0),
    };
    let Some(offset) = offset else {
        return HttpResponse::BadRequest().finish();
    };

    let quotes: Vec<Quote> =
        sqlx::query_as("SELECT * FROM quotes ORDER BY created_at ASC LIMIT 4 OFFSET $1")
            .bind(offset)
            .fetch_all(&**pool)
            .await
            .expect("Failed to fetch quotes");

    let next_token = if quotes.len() == 4 {
        let new_token: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        token_store.insert(new_token.clone(), offset + 3);
        Some(new_token)
    } else {
        None
    };

    let page = offset / 3 + 1;
    let quotes = quotes.into_iter().take(3).collect();
    let list = List {
        next_token,
        page,
        quotes,
    };

    HttpResponse::Ok().json(list)
}
