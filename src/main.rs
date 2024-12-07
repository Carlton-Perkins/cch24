mod day_start;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(day_start::day_start);
        cfg.service(day_start::seek);
    };

    Ok(config.into())
}
