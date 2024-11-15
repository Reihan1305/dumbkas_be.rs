use crate::config::db::establish_connection;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use crate::modules::users::user_model::NewUser;
use argon2::{
    password_hash::{
        rand_core::OsRng,
         PasswordHasher, SaltString
    },
    Argon2
};

pub async fn register(new_user: web::Json<NewUser>) -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;
    
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(new_user.password.as_bytes(), &salt);

    let password_hash = match password_hash {
        Ok(hash) => hash.to_string(),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("Error hashing password: {}", e))),
    };

    let mut new_user = new_user.into_inner();
    new_user.password = password_hash;

    let mut connection = establish_connection();

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error inserting new user");

    Ok(HttpResponse::Ok().json("Data inserted into the database"))
}
