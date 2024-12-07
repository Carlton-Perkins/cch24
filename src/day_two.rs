use actix_web::{get, web::Query, HttpResponse, Responder};
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Deserialize)]
struct Task1Args {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

#[get("/2/dest")]
pub async fn task_1(args: Query<Task1Args>) -> impl Responder {
    let from = args.from.octets();
    let key = args.key.octets();
    let mut dest = [0; 4];
    for i in 0..4 {
        dest[i] = from[i].wrapping_add(key[i]);
    }
    let dest = Ipv4Addr::from(dest);

    HttpResponse::Ok().body(dest.to_string())
}

#[derive(Deserialize)]
struct Task2Args {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

#[get("/2/key")]
pub async fn task_2(args: Query<Task2Args>) -> impl Responder {
    let from = args.from.octets();
    let to = args.to.octets();
    let mut dest = [0; 4];
    for i in 0..4 {
        dest[i] = to[i].wrapping_sub(from[i]);
    }
    let dest = Ipv4Addr::from(dest);

    HttpResponse::Ok().body(dest.to_string())
}

#[derive(Deserialize)]
struct Task3DestArgs {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

#[get("/2/v6/dest")]
pub async fn task_3_dest(args: Query<Task3DestArgs>) -> impl Responder {
    let from = args.from.octets();
    let key = args.key.octets();
    let mut dest = [0; 16];
    for i in 0..16 {
        dest[i] = from[i] ^ key[i];
    }
    let dest = Ipv6Addr::from(dest);

    HttpResponse::Ok().body(dest.to_string())
}

#[derive(Deserialize)]
struct Task4KeyArgs {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

#[get("/2/v6/key")]
pub async fn task_3_key(args: Query<Task4KeyArgs>) -> impl Responder {
    let from = args.from.octets();
    let to = args.to.octets();
    let mut dest = [0; 16];
    for i in 0..16 {
        dest[i] = from[i] ^ to[i];
    }
    let dest = Ipv6Addr::from(dest);

    HttpResponse::Ok().body(dest.to_string())
}
