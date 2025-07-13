use diesel::{PgConnection, QueryResult, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use crate::payments::models::Payments;
use crate::schemas::payments::dsl::payments;

pub fn list(pool: Pool<ConnectionManager<PgConnection>>) -> QueryResult<Vec<Payments>> {
    let mut conn = pool.get().unwrap();
    payments.load::<Payments>(&mut conn)
}