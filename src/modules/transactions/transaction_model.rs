use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::{sql_types::TransactionType, transactions};
use uuid::Uuid;

#[derive(Debug, Queryable,Deserialize,Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub total_transaction: i32,
    pub type_transaction: String,
    pub description: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable,Insertable,Deserialize,Serialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub user_id: Uuid,
    pub total_transaction: i32,
    pub type_transaction:TransactionType,
    pub description: String
}

#[derive(Deserialize,AsChangeset)]
#[diesel(table_name = transactions)]
pub struct UpdateTransaction {
    pub total_transaction: Option<i32>,
    pub type_transaction: Option<TransactionType>,
    pub description: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}