use rusqlite::{Connection, Error};

pub fn open_db(path: &str) -> Result<Connection, Error> {
    Connection::open(path)
}
