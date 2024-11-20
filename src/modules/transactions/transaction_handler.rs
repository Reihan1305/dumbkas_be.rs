use crate::{config::db::establish_connection, modules::transactions::transaction_model::{NewTransaction, Transaction}};
use actix_web::{dev::ServiceRequest, web, HttpMessage, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;


pub async fn create_transaction(new_transaction: web::Json<NewTransaction>, req: ServiceRequest) -> Result<HttpResponse> {
    use crate::schema::transactions::dsl::*;

    let userid:Uuid = match req.extensions().get::<Uuid>() {
        Some(uid) => uid.clone(),
        None => return Ok(HttpResponse::Unauthorized().json("User not authenticated")),
    };

    let mut new_transaction = new_transaction.into_inner();
    new_transaction.user_id = userid;

    let mut connection = establish_connection();

    let inserted_transaction: Result<Transaction, diesel::result::Error> = diesel::insert_into(transactions)
    .values(&new_transaction)
    .returning((id,user_id,total_transaction,type_transaction,description,created_at,updated_at))
    .get_result(&mut connection);

    match inserted_transaction {
        Ok(transaction) => {
            Ok(HttpResponse::Ok().json(json!({
                "message": "Transaction created successfully",
                "data": transaction
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Error creating transaction: {}", e))),
    }
}
