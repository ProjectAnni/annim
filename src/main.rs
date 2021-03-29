mod db;
mod config;
mod services;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use crate::config::AnnivConfig;

pub struct AppState {
    pool: db::AnnivPool,
    config: AnnivConfig,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

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
            )
    })
        .bind("localhost:6655")?
        .run()
        .await?;
    Ok(())
}
