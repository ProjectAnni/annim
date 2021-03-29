use actix_web::{web, Responder, get};
use serde::{Serialize, Deserialize};
use crate::AppState;
use crate::services::response::AnnivResponse;

#[derive(Serialize, Deserialize)]
pub struct AnnivSiteInfo {
    #[serde(rename(serialize = "site_name"))]
    name: String,
    description: String,
    #[serde(skip_deserializing, rename = "protocol_version", default = "protocol_version")]
    version: String,
    features: Vec<String>,
}

fn protocol_version() -> String {
    "1".to_owned()
}

#[get("/info")]
pub async fn info(state: web::Data<AppState>) -> impl Responder {
    AnnivResponse::data(&state.clone().config.site_info)
}