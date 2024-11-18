use actix_web::{web,http::header::HeaderValue};
use jsonwebtoken::{decode,encode, DecodingKey, TokenData, Validation};

use crate::config::db::establish_connection;
use crate::modules::users::user_model::UserToken;

pub fn decodeToken (token:String) -> Result<TokenData<UserToken>, String> {
    let validation = Validation::default();
    let key = DecodingKey::from_secret("secret".as_bytes());
    let token_data = decode::<UserToken>(&token, &key, &validation);
    match token_data {
        Ok(token_data) => Ok(token_data),
        Err(err) => Err(err.to_string())
    }
}


