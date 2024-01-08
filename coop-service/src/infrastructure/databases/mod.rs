use std::env;

use dotenvy::dotenv;

pub mod postgre;

pub fn get_connection_string(keyword: String) -> String {
    dotenv().ok();
    env::var(keyword).expect("DATABASE_URL must be set")
}
