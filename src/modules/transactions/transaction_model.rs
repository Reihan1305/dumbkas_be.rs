use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::transactions;
use uuid::Uuid;

#[derive(Debug, diesel_derive_enum::DbEnum,Deserialize,Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::Transactiontype"]

pub enum TransactionType {
    Debit,
    Credit,
}

#[derive(Debug, Queryable,Deserialize,Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub total_transaction: i32,
    pub type_transaction: TransactionType,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable,Deserialize,Insertable,Serialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub user_id: Option<Uuid>,
    pub total_transaction: i32,
    pub type_transaction:TransactionType,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

// #[derive(Deserialize,Queryable)]
// #[diesel(table_name = transactions)]
// pub struct UpdateTransaction {
//     pub total_transaction: Option<i32>,
//     pub type_transaction: Option<String>,
//     pub description: Option<String>,
//     pub updated_at: chrono::NaiveDateTime,
// }

