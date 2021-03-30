mod db;
mod config;
mod services;
mod models;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use crate::config::AnnivConfig;
use log::LevelFilter;

pub struct AppState {
    pool: db::AnnivPool,
    config: AnnivConfig,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_env("ANNI_LOG")
        .init();

    let config = AnnivConfig::from_file("config.toml")?;
    let state = web::Data::new(AppState {
        pool: db::AnnivPool::new(&config.database).await?,
        config,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(services::info::info)
                    .service(services::user::register)
                    .service(services::user::register_check)
            )
    })
        .bind("localhost:6655")?
        .run()
        .await?;
    Ok(())
}
