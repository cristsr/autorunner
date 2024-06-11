use crate::config;
use sqlite::{Connection, Error};
use std::process;

pub fn check() {
    if get_connection().is_err() {
        println!("Database is not connected!");
        process::exit(1);
    }

    println!("Database is connected!");
}

pub fn get_connection() -> Result<Connection, Error> {
    let db_url = config::get("DB_URL");
    let conn = Connection::open(db_url)?;
    Ok(conn)
}
