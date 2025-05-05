use crate::db::db_insert;

use super::{db_select, db_utils};
use std::io::Error;
use text_io::read;

pub fn db_main() -> Result<(), Error> {
    let conn = db_utils::open_db("sqlite.db").expect("Could not open connection to database");

    println!("\nChoose your query:\n");
    let queries = vec![
        "0. Exit",
        "1. Insert Admin",
        "2. Select user usernames",
        "3. Select user by email",
        "4. Select admins",
        "5. Insert Collaborator",
    ];
    for s in &queries {
        println!("{}", s);
    }
    print!("Your choice (0..{}): ", queries.len() - 1);
    let e = match read!() {
        0 => Ok(()),
        1 => {
            let _ = db_insert::dev_insert_admin(&conn);
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
        4 => {
            let admins = db_select::select_admin_all(&conn);
            println!("{:?}", admins);
            Ok(())
        }
        5 => {
            let _ = db_insert::dev_insert_collaborator(&conn);
            Ok(())
        }
        _ => Ok(()),
    };
    match e {
        Err(e) => Err(e),
        Ok(_) => Ok(()),
    }
}
