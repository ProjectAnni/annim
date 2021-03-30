use crate::db::AnnivPool;
use sqlx::{Executor, Any};
use crate::models::response::Error;
use sqlx::any::AnyKind;

impl AnnivPool {
    pub async fn create_table_2fa(&self) -> Result<(), Error> {
        match self.kind {
            AnyKind::Postgres => unimplemented!(),
            AnyKind::MySql => {
                sqlx::query(r#"
                CREATE TABLE IF NOT EXISTS anniv_2fa (
                    `id`       INT NOT NULL AUTO_INCREMENT,
                    `user_id`  CHAR(36) NOT NULL,
                    `secret`   VARCHAR(32) NOT NULL,
                    PRIMARY KEY(`id`)
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

    pub async fn create_2fa(executor: impl Executor<'_, Database=Any>, user_id: &str, secret: &str) -> Result<(), Error> {
        sqlx::query(r#"INSERT INTO anniv_2fa(`user_id`, `secret`) VALUES (?, ?);"#)
            .bind(user_id)
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