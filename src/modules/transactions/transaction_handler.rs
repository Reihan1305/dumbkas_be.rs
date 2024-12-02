use crate::{config::db::establish_connection, modules::transactions::transaction_model::{NewTransaction,TransactionType, Transaction}};
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use crate::Authentication;
use crate::schema::users;
use uuid::Uuid;
use crate::modules::users::user_model::User;

#[post("")]
pub async fn create_transaction(mut new_transaction: web::Json<NewTransaction>,req:HttpRequest) -> Result<HttpResponse> {
    use crate::schema::transactions::dsl::*;
    let mut connection = establish_connection();

    let userid = req
    .extensions()
    .get::<Uuid>()
    .cloned()
    .unwrap();

    let user_login:Result<Option<User>,HttpResponse> = users::table
                .find(userid)
                .first::<User>(&mut connection)
                .optional()
                .map_err(|e| {
                    HttpResponse::InternalServerError().json(json!({
                        "error": format!("Database error: {}", e)
                    }))
                });
    if user_login.is_err(){
        return Ok(HttpResponse::NotFound().json(json!({
            "error": "User not found"
        })));
    };

    new_transaction.user_id = Some(userid);

    match new_transaction.type_transaction{
        TransactionType::Credit | TransactionType::Debit => () ,
        _ => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "Invalid transaction type"
            })));
        }
    };

    let new_transaction = new_transaction.into_inner();
    
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

pub fn config(conf:&mut web::ServiceConfig){
    let scope = web::scope("/transaction")
    .wrap(Authentication)
    .service(create_transaction);

    conf.service(scope);
}
