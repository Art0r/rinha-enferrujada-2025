use actix_web::{web};
use crate::payments::views::PaymentsViews;

pub fn payment_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/payments")
            .route("", web::get().to(PaymentsViews::list))
    );
}