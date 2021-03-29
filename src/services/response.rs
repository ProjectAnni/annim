use actix_web::{error, HttpResponse};
use actix_web::http::StatusCode;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    FatalError,
    DatabaseConnectionError,

    DatabaseInsertError,
    DatabaseReadError,

    ContentNotFound,
    NoPermission,
    InvalidParameters,

    InMaintainance,

    UsernameUnavailable,
    EmailUnavailable,
    WrongEmailOrPassword,
    NoUserForRevoke,

    MaximiumPlaylistCount,
    MaximiumPlaylistSongCount,
    PlaylistNotFound,
    InvalidPlaylistModifyCommand,
    InvalidPlaylistMusicId,

    MaximiumTokenCount,
    TokenNotFound,
}

impl Error {
    fn as_u32(&self) -> u32 {
        match self {
            Error::FatalError => 900000,
            Error::DatabaseConnectionError => 900001,

            Error::DatabaseInsertError => 901000,
            Error::DatabaseReadError => 901001,

            Error::ContentNotFound => 902000,
            Error::NoPermission => 902001,
            Error::InvalidParameters => 902002,

            Error::InMaintainance => 101000,

            Error::UsernameUnavailable => 102000,
            Error::EmailUnavailable => 102001,
            Error::WrongEmailOrPassword => 102010,
            Error::NoUserForRevoke => 102020,

            Error::MaximiumPlaylistCount => 103000,
            Error::MaximiumPlaylistSongCount => 103001,
            Error::PlaylistNotFound => 103002,
            Error::InvalidPlaylistModifyCommand => 103003,
            Error::InvalidPlaylistMusicId => 103003,

            Error::MaximiumTokenCount => 104000,
            Error::TokenNotFound => 104001,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse {
        AnnivResponse::error(self.clone())
    }
}

#[derive(Serialize)]
pub struct AnnivResponse<T> {
    status: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> AnnivResponse<T> {
    pub fn data(data: T) -> HttpResponse {
        HttpResponse::Ok().json(AnnivResponse {
            status: 0,
            message: None,
            data: Some(data),
        })
    }
}

impl AnnivResponse<()> {
    pub fn ok() -> HttpResponse {
        HttpResponse::NoContent().finish()
    }

    pub fn error(error: Error) -> HttpResponse {
        let body: AnnivResponse<()> = AnnivResponse {
            status: error.as_u32(),
            message: None,
            data: None,
        };
        HttpResponse::Ok().json(body)
    }

    pub fn error_message(error: Error, message: String) -> HttpResponse {
        let body: AnnivResponse<()> = AnnivResponse {
            status: error.as_u32(),
            message: Some(message),
            data: None,
        };
        HttpResponse::Ok().json(body)
    }
}