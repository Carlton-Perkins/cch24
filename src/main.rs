mod day_five;
mod day_nine;
mod day_start;
mod day_twelve;
mod day_two;

use actix_web::{
    middleware::Logger,
    web::{Data, PathConfig, ServiceConfig},
    HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let milk_crate = Data::new(day_nine::MilkCrate::new());
    let board_data = Data::new(day_twelve::board_data());
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            actix_web::Scope::new("")
                .wrap(Logger::default())
                .app_data(PathConfig::default().error_handler(|err, _req| {
                    actix_web::error::InternalError::from_response(
                        err,
                        HttpResponse::BadRequest().into(),
                    )
                    .into()
                }))
                .service(day_start::day_start)
                .service(day_start::seek)
                .service(day_two::task_1)
                .service(day_two::task_2)
                .service(day_two::task_3_dest)
                .service(day_two::task_3_key)
                .service(day_five::task_1)
                .app_data(milk_crate)
                .service(day_nine::milk)
                .service(day_nine::refill)
                .app_data(board_data)
                .service(day_twelve::board)
                .service(day_twelve::reset)
                .service(day_twelve::place)
                .service(day_twelve::random_board),
        );
    };

    Ok(config.into())
}
