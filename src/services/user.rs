use actix_web::{web, Responder, post};
use crate::AppState;
use crate::db::AnnivPool;
use crate::models::response::{AnnivResponse, Error};
use crate::models::features::{FEATURE_2FA, FEATURE_CLOSE, FEATURE_INVITE};
use crate::models::user::{UserRegisterRequest, UserRegisterCheckRequest};

#[post("/user/register")]
pub async fn register(register: web::Json<UserRegisterRequest>, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    // if feature[close] is enabled and feature[invite] is not enabled
    let invitor = if state.config.has_feature(FEATURE_CLOSE) && !state.config.has_feature(FEATURE_INVITE) {
        return Err(Error::RegisterClosed);
    } else if state.config.has_feature(FEATURE_INVITE) {
        // if feature[invite] is enabled
        // check invite code
        match register.invite_code() {
            // invite code must be provided
            None => return Err(Error::InvalidInviteCode),
            Some(code) => {
                // validate code, invitee and use_left
                Some(state.pool.invite_validate_invitor(register.email(), code).await?)
            }
        }
    } else if !register.invite_code().is_none() {
        // if site is open and invite code is provided
        return Err(Error::InviteSystemNotEnabled);
    } else {
        None
    };

    state.pool.email_username_used(Some(register.email()), Some(register.username())).await?;

    // if feature[2fa] is enabled
    let secret_2fa = if state.config.has_feature(FEATURE_2FA) {
        match register.secret_2fa() {
            // 2fa secret must be provided
            None => return Err(Error::Invalid2FASecret),
            Some(_2fa) => {
                // and the secret should be longer than 128 bytes(16 characters)
                if _2fa.len() < 16 {
                    return Err(Error::Invalid2FASecret);
                }
                Some(_2fa)
            }
        }
    } else if !register.secret_2fa().is_none() {
        // if feature[2fa] is not enabled and 2fa_secret is provided
        // return error: 2fa not enabled
        return Err(Error::NotEnabled2FA);
    } else {
        None
    };

    // use transaction here to make sure user is registered and 2fa is enabled
    let mut tr = state.pool.pool().begin().await.map_err(|_| Error::DatabaseConnectionError)?;
    if let Some(code) = register.invite_code() {
        // use invite code
        AnnivPool::invite_use(&mut tr, code.as_ref()).await?;
    }
    // create user and return user info
    AnnivPool::create_user(&mut tr, register.username(), register.password(), register.email(), register.nickname(), register.avatar(), invitor.as_deref()).await?;
    let user = state.pool.query_user(register.email()).await?;
    if let Some(secret) = secret_2fa {
        // create 2fa
        AnnivPool::create_2fa(&mut tr, user.id(), secret).await?;
    }
    tr.commit().await.map_err(|_| Error::DatabaseWriteError)?;
    Ok(AnnivResponse::data(user))
}

#[post("/user/register/check")]
pub async fn register_check(check: web::Json<UserRegisterCheckRequest>, state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let check = check.into_inner();

    if check.email().is_none() && check.username().is_none() {
        return Err(Error::InvalidParameters);
    }

    state.pool.email_username_used(
        check.email(),
        check.username(),
    ).await?;
    Ok(AnnivResponse::ok())
}