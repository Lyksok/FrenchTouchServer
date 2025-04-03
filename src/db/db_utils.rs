use rusqlite::{Connection, Error};

pub fn open_db(path: &str) -> Result<Connection, Error> {
    Ok(Connection::open(path)?)
}


