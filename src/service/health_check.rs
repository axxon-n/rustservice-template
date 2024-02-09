use crate::common::AppState;
use actix_web::{
    web, 
    Error, 
    HttpRequest, 
    HttpResponse
};
use serde_json::json;
use anyhow::Result;

pub async fn index(_request: HttpRequest, _body: web::Bytes, _state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json").body(json!({
            "is_success": true
        }).to_string()))
}