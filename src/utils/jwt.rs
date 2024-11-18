use jsonwebtoken::{Header, encode, EncodingKey};

use crate::modules::users::user_model::UserToken;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
    pub iat : i64,
    pub exp : i64,
    pub user :UserToken,
}

impl TokenClaims {
    pub fn generate_token(data:UserToken) -> Result<String, String> {
        let max_age:i64 = 60 * 60 * 24;
        let iat = chrono::Utc::now().timestamp();
        let exp = iat + max_age;
        let token = TokenClaims {
            iat,
            exp,
            user: data,
        };

        let jwt_secret:EncodingKey = EncodingKey::from_secret("secret_key".as_bytes());
        let token = encode( &Header::default(),&token, &jwt_secret).unwrap();
        Ok(token)
    }
}


// pub fn decodeToken (token:String) -> Result<TokenData<UserToken>, String> {
//     let validation = Validation::default();
//     let key = DecodingKey::from_secret("secret".as_bytes());
//     let token_data = decode::<UserToken>(&token, &key, &validation);
//     match token_data {
//         Ok(token_data) => Ok(token_data),
//         Err(err) => Err(err.to_string())
//     }
// }

// pub fn verifyToken (
//     token_data:&TokenData<UserToken>,
//     pool: &web::Data<Pool>,
// ) -> Result <String, String> {
    
// }
