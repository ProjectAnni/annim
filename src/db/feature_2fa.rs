use crate::db::AnnivPool;
use sqlx::{Executor, Any};
use crate::services::response::Error;
use sqlx::any::AnyKind;

impl AnnivPool {
    pub async fn create_table_2fa(&self) -> Result<(), Error> {
        match self.kind {
            AnyKind::Postgres => unimplemented!(),
            AnyKind::MySql => {
                sqlx::query(r#"
                CREATE TABLE IF NOT EXISTS anniv_2fa (
                    user_id  CHAR(36) PRIMARY KEY,
                    secret   VARCHAR(32) NOT NULL
                ) DEFAULT CHARSET=utf8mb4;
                "#).execute(&self.pool).await.map_err(|e| {
                    log::error!("{:?}", e);
                    Error::DatabaseWriteError
                })?;
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    pub async fn create_2fa(executor: impl Executor<'_, Database=Any>, id: &str, secret: &str) -> Result<(), Error> {
        sqlx::query(r#"INSERT INTO anniv_2fa(user_id, secret) VALUES (?, ?);"#)
            .bind(id)
            .bind(secret)
            .execute(executor)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                Error::DatabaseWriteError
            })?;
        Ok(())
    }
}