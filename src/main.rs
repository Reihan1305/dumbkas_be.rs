mod modules{
    pub mod users {
        pub mod user_handler;
        pub mod user_model;
    }
}
mod schema;
mod config;
use actix_web::{web, App, HttpServer};
use crate::modules::users::user_handler::register;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/human", web::post().to(register))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
