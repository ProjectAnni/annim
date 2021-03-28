use actix_web::{HttpRequest, web, Responder, get};
use crate::AppState;

#[get("/info")]
async fn audio(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    //
    ""
}