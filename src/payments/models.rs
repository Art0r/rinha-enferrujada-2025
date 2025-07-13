use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schemas::payments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Payments {
    pub id: Uuid,
    pub correlationid: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub requestedat: DateTime<Utc>,
    pub amount: String,
}