use crate::payments::models::Payments;
use crate::schemas::payments::dsl::payments;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use memcache::Client;
use tokio::time::{Duration, sleep};

pub struct PaymentServices {}

async fn background() {
    sleep(Duration::from_secs(1)).await;
    println!("Tokio background task running");
}

impl PaymentServices {
    pub fn list(pool: Pool<ConnectionManager<PgConnection>>) -> QueryResult<Vec<Payments>> {
        tokio::spawn(background());
        let mut conn = pool.get().unwrap();
        payments.load::<Payments>(&mut conn)
    }
}

