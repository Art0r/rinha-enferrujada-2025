// @generated automatically by Diesel CLI.

diesel::table! {
    payments (id) {
        id -> Uuid,
        correlationid -> Uuid,
        requestedat -> Timestamptz,
        amount -> Varchar,
    }
}
