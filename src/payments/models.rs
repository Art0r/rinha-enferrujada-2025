use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable, Selectable, prelude::Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schemas::payments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Payments {
    pub id: i64,
    pub correlationid: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub requestedat: DateTime<Utc>,
    pub amount: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schemas::payments)]
pub struct NewPayment {
    pub correlationid: Uuid,
    #[serde(skip_deserializing)]
    pub requestedat: DateTime<Utc>,
    pub amount: String,
}

impl NewPayment {
    pub fn new(correlationid: Uuid, amount: String) -> Self {
        Self {
            correlationid,
            requestedat: Utc::now(),
            amount,
        }
    }
}
