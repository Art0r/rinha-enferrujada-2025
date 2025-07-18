use crate::payments::views::PaymentsViews;
use actix_web::web;

pub fn payment_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/payments")
            .route("", web::get().to(PaymentsViews::list))
            .route("", web::post().to(PaymentsViews::create)),
    );
}
