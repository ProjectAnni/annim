use crate::db::AnnivPool;
use sqlx::any::AnyKind;
use crate::services::response::Error;

impl AnnivPool {
    pub async fn create_table_user(&self) -> Result<(), Error> {
        match self.kind {
            AnyKind::Postgres => {
                sqlx::query(r#"
                CREATE TABLE IF NOT EXISTS anni_user (
                    id              TEXT PRIMARY KEY,
                    inviter_id      TEXT NOT NULL,
                    nickname        TEXT NOT NULL,
                    username        TEXT NOT NULL UNIQUE,
                    email           TEXT NOT NULL UNIQUE,
                    password        TEXT NOT NULL,
                    avatar          TEXT
                );
                "#)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| {
                        log::error!("{:?}", e);
                        Error::DatabaseInsertError
                    })?;
            }
            AnyKind::MySql => {
                sqlx::query(r#"
                CREATE TABLE IF NOT EXISTS anniv_user (
                    `id`         CHAR(36) PRIMARY KEY DEFAULT UUID(),
                    `inviter_id` CHAR(36) DEFAULT '5e9d2c21-963f-52c3-b832-fd4d3adc96cd', -- default inviter is uuidv5(ns:DNS, 'anni.mmf.moe')
                    `nickname`   VARCHAR(32) NOT NULL,
                    `username`   VARCHAR(32) NOT NULL UNIQUE,
                    `email`      VARCHAR(64) NOT NULL UNIQUE,
                    `password`   VARCHAR(32) NOT NULL,
                    `avatar`     VARCHAR(256) NOT NULL,

                    `register_at` TIMESTAMP NOT NULL
                ) DEFAULT CHARSET=utf8mb4;
                "#)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| {
                        log::error!("{:?}", e);
                        Error::DatabaseInsertError
                    })?;
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    pub async fn create_user(&self, username: &str, password: &str, email: &str, nickname: &str, avatar: &str) -> Result<(), Error> {
        sqlx::query(r#"
        INSERT INTO anniv_user(`email`, `username`, `password`, `nickname`, `avatar`) VALUES (?, ?, ?, ?, ?);
        "#)
            .bind(email)
            .bind(username)
            .bind(password)
            .bind(nickname)
            .bind(avatar)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                log::error!("{:?}", e);
                Error::DatabaseInsertError
            })?;
        Ok(())
    }

    pub async fn email_username_used(&self, email: Option<&str>, username: Option<&str>) -> Result<(), Error> {
        if let Some(email) = email {
            let (count, ): (i64, ) = sqlx::query_as(r#"
        SELECT count(*) FROM anniv_user WHERE email = ?
        "#)
                .bind(email)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    Error::DatabaseReadError
                })?;
            if count > 0 {
                return Err(Error::EmailUnavailable);
            }
        }

        if let Some(username) = username {
            let (count, ): (i64, ) = sqlx::query_as(r#"
        SELECT count(*) FROM anniv_user WHERE username = ?
        "#)
                .bind(username)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| {
                    log::error!("{:?}", e);
                    Error::DatabaseReadError
                })?;
            if count > 0 {
                return Err(Error::UsernameUnavailable);
            }
        }

        Ok(())
    }
}