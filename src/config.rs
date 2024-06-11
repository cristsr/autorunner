use dotenv::dotenv;
use std::env;

pub fn get(key: &str) -> String {
    dotenv().ok();
    env::var(key).expect(format!("{key} must be set").as_str())
}
