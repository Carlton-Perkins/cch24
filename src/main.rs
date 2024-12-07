mod day_five;
mod day_start;
mod day_two;

use actix_web::{middleware::Logger, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            actix_web::Scope::new("")
                .wrap(Logger::default())
                .service(day_start::day_start)
                .service(day_start::seek)
                .service(day_two::task_1)
                .service(day_two::task_2)
                .service(day_two::task_3_dest)
                .service(day_two::task_3_key)
                .service(day_five::task_1),
        );
    };

    Ok(config.into())
}
