use actix_web::{web, Responder, get};
use crate::AppState;
use crate::models::response::AnnivResponse;

#[get("/info")]
pub async fn info(state: web::Data<AppState>) -> impl Responder {
    AnnivResponse::data(&state.clone().config.site_info)
}