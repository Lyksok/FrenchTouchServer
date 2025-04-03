use std::io::Error;
use super::db_utils;
use super::db_getter;

pub fn db_main() -> Result<(), Error> {
    let conn = db_utils::open_db("sqlite.db");
    let usernames = db_getter::get_user_usernames(conn.expect("Could not open connection to database"));

    Ok(println!("{:?}",usernames))
}
