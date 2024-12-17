use std::{cell::LazyCell, collections::HashMap, ops::Deref, sync::Mutex};

use actix_web::{
    cookie::Cookie,
    get, post,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use jsonwebtoken::{encode, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    id: Uuid,
}

const SECRET: &[u8] = b"super secure secret not to be used in production :P";
const ENCODE_KEY: LazyCell<jsonwebtoken::EncodingKey> =
    LazyCell::new(|| jsonwebtoken::EncodingKey::from_secret(SECRET));
const DECODE_KEY: LazyCell<jsonwebtoken::DecodingKey> =
    LazyCell::new(|| jsonwebtoken::DecodingKey::from_secret(SECRET));

#[post("/16/wrap")]
pub async fn wrap(data: Json<Value>, store: Data<GiftStore>) -> impl Responder {
    let id = Uuid::new_v4();
    let claims = Claims { id };
    let jwt = encode(&Header::default(), &claims, &ENCODE_KEY).unwrap();
    store.lock().unwrap().insert(id, data.into_inner());

    HttpResponse::Ok().cookie(Cookie::new("gift", jwt)).finish()
}

#[get("/16/unwrap")]
pub async fn unwrap(req: HttpRequest, store: Data<GiftStore>) -> impl Responder {
    let Some(cookie) = req.cookie("gift") else {
        println!("No cookie found");
        return HttpResponse::BadRequest().finish();
    };
    let jwt = cookie.value();
    let mut vaidation = Validation::new(jsonwebtoken::Algorithm::HS256);
    vaidation.required_spec_claims.clear();

    let Ok(claims) = jsonwebtoken::decode::<Claims>(&jwt, &DECODE_KEY, &vaidation) else {
        println!("Failed to decode JWT");
        return HttpResponse::BadRequest().finish();
    };

    let store = store.lock().unwrap();
    let Some(data) = store.get(&claims.claims.id) else {
        println!("No data found");
        return HttpResponse::BadRequest().finish();
    };

    HttpResponse::Ok().body(data.to_string())
}

pub struct GiftStore(Mutex<HashMap<Uuid, Value>>);

impl GiftStore {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

impl Deref for GiftStore {
    type Target = Mutex<HashMap<Uuid, Value>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
