use actix_web::{get, web, HttpResponse};
use crate::AppState;
use crate::payments::services::list;

#[get("/")]
pub async fn list_payments(data: web::Data<AppState>) -> HttpResponse {
    match list(data.pool.clone()) {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}