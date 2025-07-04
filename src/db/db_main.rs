use crate::db::{db_insert, db_search};

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
        "6. Search Songs and Artists",
        "7. Select all artist",
        "8. Select all albums",
        "9. Insert song album",
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
        6 => {
            let a = db_search::select_search_song(&conn, "91");
            let b = db_search::select_search_artist(&conn, "pnl");
            println!("91: {:?}",a);
            println!("pnl: {:?}",b);
            Ok(())
        }
        7 => {
            let a = db_select::select_artist_all(&conn);
            println!("Artists: {:?}",a);
            Ok(())
        }
        8 => {
            let a = db_select::select_album_all(&conn);
            println!("Albums: {:?}",a);
            Ok(())
        }
        9 => {
            let a = db_insert::dev_insert_song_album(&conn);
            println!("SongAlbum: {:?}",a);
            Ok(())
        }
        _ => Ok(()),
    };
    match e {
        Err(e) => Err(e),
        Ok(_) => Ok(()),
    }
}
