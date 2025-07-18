use chrono::Utc;
use diesel::result::Error;

use crate::payments::forms::NewPaymentForm;
use crate::payments::models::{NewPayment, Payments};
use crate::schemas::payments::dsl::payments;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, QueryResult, RunQueryDsl, SelectableHelper};
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

    pub fn create(
        pool: Pool<ConnectionManager<PgConnection>>,
        form: NewPaymentForm,
    ) -> Result<Payments, Error> {
        let mut conn = pool.get().expect("asdk");
        let new_payment = NewPayment {
            correlationid: form.correlationid,
            amount: form.amount,
            requestedat: Utc::now(),
        };

        diesel::insert_into(payments)
            .values(&new_payment)
            .returning(Payments::as_returning())
            .get_result(&mut conn)
            .map_err(Error::from)
    }
}
