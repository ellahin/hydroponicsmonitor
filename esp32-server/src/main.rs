mod repo;

use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().expect("Error no .env");

    if env::var("DATABASE_URL").is_err() {
        panic!("DATABASE_URL is not a variable");
    }
}
