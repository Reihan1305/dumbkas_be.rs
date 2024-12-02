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

use crate::middlewares::auth_middleware::Authentication;
use actix_web::{App,HttpServer};
use modules::{transactions::transaction_handler, users::user_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(user_handler::user_config)
            .configure(transaction_handler::config)
            })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
