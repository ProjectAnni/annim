use actix_web::{error, HttpResponse};
use actix_web::http::{StatusCode, header};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    FatalError,
    DatabaseConnectionError,

    DatabaseInsertError,
    DatabaseReadError,

    ContentNotFound,
    NoPermission,

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
        let body: AnnivResponse<u8> = AnnivResponse {
            status: self.as_u32(),
            message: None,
            data: None,
        };
        HttpResponse::Ok().json(body)
    }
}

#[derive(Serialize)]
pub(crate) struct AnnivResponse<T> {
    status: u32,
    message: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> AnnivResponse<T> {
    pub(crate) fn Ok(data: Option<T>) -> HttpResponse {
        match data {
            None => HttpResponse::NoContent().finish(),
            Some(data) => {
                HttpResponse::Ok().json(AnnivResponse {
                    status: 0,
                    message: None,
                    data: Some(data),
                })
            }
        }
    }
}