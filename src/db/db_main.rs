use super::{db_create, db_insert, db_select, db_utils};
use std::io::Error;
use text_io::read;

pub fn db_main() -> Result<(), Error> {
    let conn = db_utils::open_db("sqlite.db").expect("Could not open connection to database");

    println!("\nChoose your query:\n");
    let queries = vec![
        "0. Exit",
        "1. Create database",
        "2. Select user usernames",
        "3. Select user by email",
        "4. Insert user in database",
    ];
    for s in &queries {
        println!("{}", s);
    }
    print!("Your choice (0..{}): ", queries.len() - 1);
    let e = match read!() {
        0 => return Ok(()),
        1 => Ok(db_create::create_db(conn)),
        2 => {
            let usernames = db_select::select_usernames(conn);
            return Ok(println!("{:?}", usernames));
        }
        3 => {
            let user = db_select::dev_select_user_by_email(conn).expect("Error with rusqlite");
            let user_json = serde_json::to_string(&user).expect("Could not parse to JSON");
            return Ok(println!("{}", user_json));
        }
        4 => Ok(db_insert::dev_insert_user(conn)),
        _ => return Ok(()),
    };
    match e {
        Err(e) => return Err(e),
        Ok(_) => return Ok(()),
    }
}
