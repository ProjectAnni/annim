use actix_web::{web, Responder, post};
use crate::AppState;
use crate::services::response::{AnnivResponse, Error};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserRegisterRequest {
    username: String,
    password: String,
    email: String,
    nickname: String,
    avatar: String,
    // TODO: 2fa
    // #[serde(rename = "2fa_secret")]
    // secret_2fa: Option<String>,
}

#[derive(Serialize)]
pub struct UserInfo {
    id: String,
    username: String,
    email: String,
    nickname: String,
    avatar: String,
}

#[post("/user/register")]
pub async fn register(register: web::Json<UserRegisterRequest>, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    state.pool.email_username_used(Some(&register.email), Some(&register.username)).await?;
    state.pool.create_user(&register.username, &register.password, &register.email, &register.nickname, &register.avatar).await?;
    Ok(AnnivResponse::ok())
}

#[derive(Deserialize)]
pub struct UserRegisterCheckRequest {
    email: Option<String>,
    username: Option<String>,
}

#[post("/user/register/check")]
pub async fn register_check(check: web::Json<UserRegisterCheckRequest>, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let check = check.into_inner();

    if check.email.is_none() && check.username.is_none() {
        return Err(Error::InvalidParameters);
    }

    state.pool.email_username_used(
        check.email.as_deref(),
        check.username.as_deref(),
    ).await?;
    Ok(AnnivResponse::ok())
}