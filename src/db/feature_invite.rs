use crate::db::AnnivPool;
use crate::models::response::Error;
use sqlx::any::AnyKind;
use sqlx::{Executor, Any};

impl AnnivPool {
    pub async fn create_table_invite(&self) -> Result<(), Error> {
        match self.kind {
            AnyKind::Postgres => unimplemented!(),
            AnyKind::MySql => {
                sqlx::query(r#"
                CREATE TABLE IF NOT EXISTS anniv_invite (
                    `id`       INT NOT NULL AUTO_INCREMENT,
                    `code`     CHAR(36) NOT NULL,
                    `invitor`  CHAR(36) NOT NULL,
                    `invitee`  VARCHAR(32),
                    `use_left` INT DEFAULT 1,
                    PRIMARY KEY(`id`),
                    KEY(`code`)
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

    pub async fn invite_validate_invitor(&self, email: &str, code: &str) -> Result<String, Error> {
        let (invitor, use_left, ): (String, i64, ) = sqlx::query_as(r#"SELECT `invitor`, `use_left` FROM anniv_invite WHERE `code` = ? AND ( ISNULL(`invitee`) OR `invitee` = ? );"#)
            .bind(code)
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                Error::InvalidInviteCode
            })?;
        if use_left == 0 {
            return Err(Error::InviteCodeMaximiumUsed);
        }

        Ok(invitor)
    }

    pub async fn invite_use(executor: impl Executor<'_, Database=Any>, code: &str) -> Result<(), Error> {
        let affected = sqlx::query(r#"UPDATE anniv_invite SET `use_left` = `use_left` - 1 WHERE `code` = ?"#)
            .bind(code)
            .execute(executor)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                Error::DatabaseWriteError
            })?;
        if affected.rows_affected() == 0 {
            return Err(Error::InviteCodeMaximiumUsed);
        }
        Ok(())
    }
}