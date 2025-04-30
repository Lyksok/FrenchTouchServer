use super::{db_create, db_select, db_utils};
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
    ];
    for s in &queries {
        println!("{}", s);
    }
    print!("Your choice (0..{}): ", queries.len() - 1);
    let e = match read!() {
        0 => Ok(()),
        1 => {
            let _ = db_create::create_db(conn);
            Ok(())
        }
        2 => {
            let usernames = db_select::select_usernames(conn);
            println!("{:?}", usernames);
            Ok(())
        }
        3 => {
            let user = db_select::dev_select_user_by_email(conn);
            println!("{:?}", user);
            Ok(())
        }
        _ => Ok(()),
    };
    match e {
        Err(e) => Err(e),
        Ok(_) => Ok(()),
    }
}
