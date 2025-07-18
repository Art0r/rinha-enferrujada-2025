use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewPaymentForm {
    pub correlationid: Uuid,
    pub amount: String,
}
