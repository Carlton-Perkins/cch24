mod day_five;
mod day_start;
mod day_two;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(day_start::day_start);
        cfg.service(day_start::seek);
        cfg.service(day_two::task_1);
        cfg.service(day_two::task_2);
        cfg.service(day_two::task_3_dest);
        cfg.service(day_two::task_3_key);
        cfg.service(day_five::task_1);
    };

    Ok(config.into())
}
