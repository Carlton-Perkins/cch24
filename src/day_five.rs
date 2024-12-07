use actix_web::{post, HttpRequest, HttpResponse, Responder};
use cargo_manifest::{Manifest, MaybeInherited};
use serde::Deserialize;

#[derive(serde::Deserialize, Debug)]
struct Order {
    item: Option<String>,
    quantity: Option<u32>,
}

#[post("/5/manifest")]
pub async fn task_1(body: String, req: HttpRequest) -> impl Responder {
    let ct = req.headers().get("Content-Type");
    let manifest = match ct {
        Some(ct) if ct == "application/json" => {
            let Ok(manifest): Result<Manifest, _> = serde_json::from_str(&body) else {
                println!("body: {body}");
                return HttpResponse::BadRequest().body("Invalid manifest");
            };
            manifest
        }
        Some(ct) if ct == "application/toml" => {
            let Ok(manifest): Result<Manifest, _> = toml::from_str(&body) else {
                println!("body: {body}");
                return HttpResponse::BadRequest().body("Invalid manifest");
            };
            manifest
        }
        Some(ct) if ct == "application/yaml" => {
            let Ok(manifest): Result<Manifest, _> = serde_yaml::from_str(&body) else {
                println!("body: {body}");
                return HttpResponse::BadRequest().body("Invalid manifest");
            };
            manifest
        }
        _ => {
            return HttpResponse::UnsupportedMediaType().finish();
        }
    };
    println!("{:?}", manifest);
    let Some(package) = manifest.package else {
        return HttpResponse::NoContent().finish();
    };
    let Some(MaybeInherited::Local(keywords)) = package.keywords else {
        return HttpResponse::BadRequest().body("Magic keyword not provided");
    };
    if !keywords.contains(&"Christmas 2024".to_string()) {
        return HttpResponse::BadRequest().body("Magic keyword not provided");
    }
    let Some(metadata) = package.metadata else {
        return HttpResponse::NoContent().finish();
    };
    let Some(orders_field) = metadata.get("orders") else {
        return HttpResponse::NoContent().finish();
    };
    let Some(orders_table) = orders_field.as_array() else {
        return HttpResponse::NoContent().finish();
    };

    let mut orders = vec![];
    for table in orders_table {
        let Ok(order): Result<Order, _> = Order::deserialize(table.clone()) else {
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
