use crate::config::db::establish_connection;
use actix_web::{web,HttpResponse,Result};
use diesel::prelude::*;
use crate::modules::users::user_model::NewUser;

pub async fn register(new_user: web::Json<NewUser>) -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();
		
    diesel::insert_into(users)
        .values(&new_user.into_inner())
        .execute(&mut connection)
        .expect("Error inserting new human");
    Ok(HttpResponse::Ok().json(
        "data inserted into the database"))
}