use actix_web::{post, HttpResponse, Responder};
use serde::Deserialize;
use toml::{value::Table, Value};

#[derive(serde::Deserialize, Debug)]
struct Toml {
    package: Package,
}

#[derive(serde::Deserialize, Debug)]
struct Package {
    name: Option<String>,
    authors: Option<Vec<String>>,
    keywords: Option<Vec<String>>,

    metadata: Option<Metadata>,
}

#[derive(serde::Deserialize, Debug)]
struct Metadata {
    orders: Vec<Value>,
}

#[derive(serde::Deserialize, Debug)]
struct Order {
    item: Option<String>,
    quantity: Option<u32>,
}

#[post("/5/manifest")]
pub async fn task_1(body: String) -> impl Responder {
    let Ok(toml): Result<Toml, _> = toml::from_str(&body) else {
        println!("body: {body}");
        return HttpResponse::NoContent().finish();
    };
    println!("{:?}", toml);
    let Some(metadata) = toml.package.metadata else {
        return HttpResponse::NoContent().finish();
    };

    let mut orders = vec![];
    for table in metadata.orders {
        let Ok(order): Result<Order, _> = Order::deserialize(table) else {
            continue;
        };
        if order.item.is_none() || order.quantity.is_none() {
            continue;
        }

        orders.push(order);
    }

    if orders.is_empty() {
        return HttpResponse::NoContent().finish();
    }

    let mut output = String::new();
    let mut first = true;
    for order in orders {
        let nl = if first {
            first = false;
            ""
        } else {
            "\n"
        };
        output.push_str(&format!(
            "{}{}: {}",
            nl,
            order.item.unwrap(),
            order.quantity.unwrap()
        ));
    }
    println!("output: {output}");

    HttpResponse::Ok().body(output)
}
