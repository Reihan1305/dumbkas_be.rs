// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransactionType;

    transactions (id) {
        id -> Uuid,
        user_id -> Uuid,
        total_transaction -> Int4,
        type_transaction -> TransactionType,
        #[max_length = 225]
        description -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 200]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    wallets (id) {
        id -> Uuid,
        user_id -> Uuid,
        total_amount -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(transactions -> users (user_id));
diesel::joinable!(wallets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    transactions,
    users,
    wallets,
);
