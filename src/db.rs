use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
                            .expect("INVALID DATABASE URL");
    
    PgConnection::establish(&database_url).expect(&format!("CONNECTION FAILED TO {}", database_url))
}