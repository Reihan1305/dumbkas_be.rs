use crate::{config::db::establish_connection, modules::transactions::transaction_model::{NewTransaction, Transaction}};
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;
use crate::Authentication;
#[post("")]
pub async fn create_transaction(new_transaction: web::Json<NewTransaction>,req:HttpRequest) -> Result<HttpResponse> {
    use crate::schema::transactions::dsl::*;
    // let mut iduser: Option<Uuid> = None;
    // println!("{:?}",req);
    // if let Some(userid) = req.extensions().get::<Uuid>() {
    //     // If userid is found, clone it into `userid`
    //     iduser = Some(userid.clone());
    // } else {
    //     // If userid is not found, return Unauthorized response and exit early
    //     HttpResponse::Unauthorized().finish();
    // }

    // // Now, `userid` is guaranteed to be initialized
    // if let Some(uod) = iduser {
    //     println!("{}",uod ); // Prints the user ID
    // }
    // new_transaction.userid = uod;
    println!("{:?}", req.extensions().get::<String>().unwrap_or(&"novalue".to_string()));
    if let Some(userid) = req.extensions().get::<String>() {
        HttpResponse::Ok().body(format!("Access granted for user: {}", userid));
    } else {
        HttpResponse::Unauthorized().body("Authorization header missing or invalid");
    }
    let new_transaction = new_transaction.into_inner();
    
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

pub fn config(conf:&mut web::ServiceConfig){
    let scope = web::scope("/transaction")
    .wrap(Authentication)
    .service(create_transaction);

    conf.service(scope);
}
