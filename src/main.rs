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

use std::env;

use crate::middlewares::auth_middleware::Authentication;
use actix_web::{web, App, HttpServer};
use diesel::{r2d2, PgConnection};
use dotenv::dotenv;
use modules::{transactions::transaction_handler, users::user_handler};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to postgres DB file");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_handler::user_config)
            .configure(transaction_handler::config)
            })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
