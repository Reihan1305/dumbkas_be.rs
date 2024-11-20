mod modules{
    pub mod users {
        pub mod user_handler;
        pub mod user_model;
    }
    pub mod transactions {
        pub mod transaction_handler;
        pub mod transaction_model;
    }
}
mod middlewares;
mod utils;
mod schema;
mod config;


use actix_web::{web, App, HttpServer};
use modules::users::user_handler::login;
use crate::modules::users::user_handler::register;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
