mod user;

use sqlx::{Pool, Any};
use sqlx::any::AnyKind;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct AnnivDbConfig {
    #[serde(default = "default_five")]
    max_connections: u32,
    uri: String,
}

fn default_five() -> u32 {
    5
}

pub struct AnnivPool {
    kind: AnyKind,
    pool: Pool<Any>,
}

impl AnnivPool {
    pub async fn new(config: &AnnivDbConfig) -> anyhow::Result<Self> {
        let pool = sqlx::any::AnyPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.uri)
            .await?;
        let pool = AnnivPool {
            kind: AnyKind::from_str(&config.uri)?,
            pool,
        };
        pool.create_table_user().await?;
        Ok(pool)
    }
}