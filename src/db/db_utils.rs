use rusqlite::{Connection, Error};
use std::path::PathBuf;

use super::db_create;

pub fn open_db(path: &str) -> Result<Connection, Error> {
    let mut conn;
    if !PathBuf::from(&path).exists() {
        println!("[DB] Creating database");
        conn = Connection::open(path);
        conn = Ok(db_create::create_db(conn.unwrap()));
    } else {
        conn = Connection::open(path);
    }
    conn
}
