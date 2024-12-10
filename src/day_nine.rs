use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use actix_web::{post, web::Data, HttpResponse, Responder};
use leaky_bucket::RateLimiter;

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

#[post("/9/milk")]
pub async fn milk(milk_crate: Data<Arc<MilkCrate>>) -> impl Responder {
    let mc = milk_crate.ratelimit.lock().unwrap();
    println!("{}", mc.balance());
    let any_left = mc.try_acquire(1);
    println!("{any_left}, {}", mc.balance());
    if !any_left {
        return HttpResponse::TooManyRequests().body("No milk available\n");
    }
    HttpResponse::Ok().body("Milk withdrawn\n")
}
