use std::env;
use diesel::{prelude::*, r2d2, Connection, PgConnection};
use dotenv::dotenv;


pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect(
                  "DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!
        ("Error connecting to the database: {}",  
                         database_url))
}