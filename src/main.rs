mod db;
mod config;
mod services;
mod models;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_session::CookieSession;
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
            .wrap(CookieSession::signed(&[0; 32])
                .name(state.config.properties.session())
                .http_only(true)
                .lazy(true)
                .secure(false)
            )
            .service(
                web::scope("/api")
                    .service(services::info::info)
                    .service(services::user::register)
                    .service(services::user::register_check)
                    .service(services::user::login)
                    .service(services::user::logout)
                    .service(services::user::revoke)
            )
    })
        .bind("localhost:6655")?
        .run()
        .await?;
    Ok(())
}
