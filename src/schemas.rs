// @generated automatically by Diesel CLI.

diesel::table! {
    payments (id) {
        id -> BigInt,
        correlationid -> Uuid,
        requestedat -> Timestamptz,
        amount -> Varchar,
    }
}
