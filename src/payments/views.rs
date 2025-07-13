use actix_web::{get, web, HttpResponse};
use crate::AppState;
use crate::payments::services::PaymentServices;

pub struct PaymentsViews {}

impl PaymentsViews {
    pub async fn list(data: web::Data<AppState>) -> HttpResponse {
        match PaymentServices::list(data.pool.clone()) {
            Ok(list) => HttpResponse::Ok().json(list),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}