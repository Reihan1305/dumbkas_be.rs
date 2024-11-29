use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid:: Uuid;
use crate::schema::users;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserToken {
    pub id : Uuid,
    pub email : String,
    pub name : String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct JwtUserToken{
        pub user: UserToken,  
        pub iat: i64,    
        pub exp: i64
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]  
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct LoginUser {
    pub email: String,
    pub password: String
}

// #[derive(Deserialize, AsChangeset)]
// #[diesel(table_name = users)]
// pub struct UpdateUser {
//     pub name: Option<String>,
//     pub email: Option<String>,
//     pub password: Option<String>,
// }
