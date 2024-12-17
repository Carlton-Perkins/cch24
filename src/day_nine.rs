use std::{sync::Mutex, time::Duration};

use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use leaky_bucket::RateLimiter;
use serde::{Deserialize, Serialize};

pub struct MilkCrate {
    pub ratelimit: Mutex<RateLimiter>,
}

impl MilkCrate {
    pub fn new() -> Self {
        println!("made new milk crate");
        Self {
            ratelimit: Mutex::new(
                RateLimiter::builder()
                    .initial(5)
                    .interval(Duration::from_secs(1))
                    .max(5)
                    .build(),
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
#[serde(rename_all = "snake_case")]
enum Convert {
    Liters(f32),
    Gallons(f32),
}

const LITER_TO_GALLON: f32 = 0.26417206;

#[post("/9/milk")]
pub async fn milk(
    body: Option<Json<Convert>>,
    milk_crate: Data<MilkCrate>,
    req: HttpRequest,
) -> impl Responder {
    let mc = milk_crate.ratelimit.lock().unwrap();
    println!("{}", mc.balance());
    let any_left = mc.try_acquire(1);
    println!("{any_left}, {}", mc.balance());
    if !any_left {
        return HttpResponse::TooManyRequests().body("No milk available\n");
    }

    match req.headers().get("Content-Type") {
        Some(ct) if ct == "application/json" => {
            let Some(convert) = body else {
                return HttpResponse::BadRequest().finish();
            };
            println!("{convert:?}");
            let convert = convert.into_inner();

            match convert {
                Convert::Liters(l) => {
                    let gallons = l as f32 * LITER_TO_GALLON;
                    let gallons = Convert::Gallons(gallons);
                    HttpResponse::Ok().body(serde_json::to_string(&gallons).unwrap())
                }
                Convert::Gallons(g) => {
                    let liters = g as f32 / LITER_TO_GALLON;
                    let liters = Convert::Liters(liters);
                    HttpResponse::Ok().body(serde_json::to_string(&liters).unwrap())
                }
            }
        }
        _ => HttpResponse::Ok().body("Milk withdrawn\n"),
    }
}
