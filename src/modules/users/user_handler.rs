use crate::{config::db::establish_connection, modules::users::user_model::{User, UserToken}};
use actix_web::{post, web, HttpResponse, Result};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::modules::users::user_model::NewUser;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2, PasswordVerifier
};
use serde_json::json;
use uuid::Uuid;

use super::user_model::LoginUser;

use crate::utils::jwt::TokenClaims;

#[derive(Serialize, Debug, Queryable, Deserialize)]
pub struct RegisterPayload {
    id: Uuid,
    name: String,
    email: String,
}

#[post("/register")]
pub async fn register(new_user: web::Json<NewUser>) -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;

    let salt: SaltString = SaltString::generate(&mut OsRng);

    let argon2: Argon2<'_> = Argon2::default();

    let password_hash: Result<_, argon2::password_hash::Error> = argon2.hash_password(new_user.password.as_bytes(), &salt);

    let password_hash: String = match password_hash {
        Ok(hash) => hash.to_string(),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(format!("Error hashing password: {}", e))),
    };

    let mut new_user = new_user.into_inner();
    new_user.password = password_hash;

    let mut connection = establish_connection();

    let inserted_user: Result<RegisterPayload, diesel::result::Error> = diesel::insert_into(users)
        .values(&new_user)
        .returning((id, name, email))
        .get_result(&mut connection);

    match inserted_user {
        Ok(user) => {
            let user_payload: RegisterPayload = RegisterPayload{
                id: user.id,
                name: user.name,
                email: user.email
            };
            Ok(HttpResponse::Ok().json(json!({
                "message": "User registered successfully",
                "data": user_payload
            })))
        }
        Err(err) => {
            if err.to_string().contains("duplicate key"){
                return Ok(HttpResponse::BadRequest().json(json!({"message": "Failed to register user","message": "email alredy exist"})))
            }
            Ok(HttpResponse::InternalServerError().json(json!({
                "message": "Failed to register user",
                "error": err.to_string()
            })))
        }
    }
}

#[post("/login")]
pub async fn login(login_data :web::Json<LoginUser>) -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    let user = users
        .filter(email.eq(&login_data.email))
        .select((id, name, email, password))
        .first::<User>(&mut connection);

    match user {
        Ok(user) => {
            let password_hash = user.password;
            let argon2 = Argon2::default();
            let parsed_hash = argon2::PasswordHash::new(&password_hash).unwrap();
            let result = argon2.verify_password(login_data.password.as_bytes(), &parsed_hash);
            match result {
                Ok(_) => {
                    let user_payload:UserToken = UserToken {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                    };
                let token:String = TokenClaims::generate_token(user_payload).unwrap();
                    Ok(HttpResponse::Ok().json(json!({
                        "message": "User logged in successfully",
                        "data": token
                    })))
                }
                Err(err) => {
                    println!("Error logging in user: {}", err.to_string().contains("duplicate key"));
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "message": "Failed to login user",
                        "error": err.to_string()
                    })))
                }
            }
        },
        Err(err) => {
            println!("Error logging in user: {}", err.to_string().contains("duplicate key"));
            Ok(HttpResponse::InternalServerError().json(json!({
                "message": "Failed to login user",
                "error": err.to_string()
            })))
        }
    }    

}

pub fn user_config(conf:&mut web::ServiceConfig){
    let scope = web::scope("/auth")
        .service(login)
        .service(register);

    conf.service(scope);
}