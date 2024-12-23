mod day_five;
mod day_nine;
mod day_nineteen;
mod day_sixteen;
mod day_start;
mod day_twelve;
mod day_two;

use actix_files::Files;
use actix_web::{
    middleware::Logger,
    web::{Data, PathConfig, ServiceConfig},
    HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let milk_crate = Data::new(day_nine::MilkCrate::new());
    let board_data = Data::new(day_twelve::board_data());
    let gift_store = Data::new(day_sixteen::GiftStore::new());
    let pool_data = Data::new(pool);
    let token_store = Data::new(day_nineteen::TokenStore::default());
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
                .service(day_twelve::random_board)
                .service(day_sixteen::wrap)
                .service(day_sixteen::unwrap)
                .app_data(gift_store)
                .service(day_sixteen::decode)
                .app_data(pool_data)
                .service(day_nineteen::reset)
                .service(day_nineteen::cite)
                .service(day_nineteen::undo)
                .service(day_nineteen::draft)
                .service(day_nineteen::remove)
                .app_data(token_store)
                .service(day_nineteen::list)
                .service(Files::new("/assets", "assets").show_files_listing()),
        );
    };

    Ok(config.into())
}
