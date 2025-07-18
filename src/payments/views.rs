use crate::payments::services::PaymentServices;
use crate::{AppState, payments::forms::NewPaymentForm};
use actix_web::{HttpResponse, get, web};

pub struct PaymentsViews {}

impl PaymentsViews {
    pub async fn list(data: web::Data<AppState>) -> HttpResponse {
        match PaymentServices::list(data.pool.clone()) {
            Ok(list) => HttpResponse::Ok().json(list),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    pub async fn create(
        data: web::Data<AppState>,
        web::Json(form): web::Json<NewPaymentForm>,
    ) -> HttpResponse {
        match PaymentServices::create(data.pool.clone(), form) {
            Ok(payment) => HttpResponse::Ok().json(payment),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}

