use super::structs::User;
use rusqlite::{params, Connection};
use text_io::read;

pub fn select_usernames(conn: Connection) -> Option<Vec<(i32, String)>> {
    let mut format = match conn.prepare("SELECT id, username, email, password_hash, password_salt, last_connection, creation_date, profile_picture FROM User") {
        Ok(fmt) => fmt,
        Err(_) => return None,
    };
    let user_iter = match format.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: String::new(),
            password_hash: String::new(),
            password_salt: String::new(),
            last_connection: 0,
            creation_date: 0,
            profile_picture: None,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut usernames = Vec::new();
    for user in user_iter {
        match user {
            Ok(user) => usernames.push((user.id, user.username)),
            Err(_) => return None,
        }
    }

    Some(usernames)
}

pub fn select_user_by_email(conn: &Connection, email: &str) -> Option<User> {
    let mut format = match conn.prepare("SELECT id,username,email,password_hash,password_salt,last_connection,creation_date,profile_picture FROM User WHERE email LIKE ?1") {
        Ok(fmt) => fmt,
        Err(_) => return None,
    };

    let user_iter = match format.query_map(params![email], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            password_salt: row.get(4)?,
            last_connection: row.get(5)?,
            creation_date: row.get(6)?,
            profile_picture: row.get(7)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut users = Vec::new();
    for user in user_iter {
        match user {
            Ok(user) => users.push(user),
            Err(_) => return None,
        }
    }
    match users.len() {
        0 => {
            println!("No user found");
            None
        }
        n => {
            println!("{} user(s) found", n);
            Some(users[0].clone())
        }
    }
}

pub fn dev_select_user_by_email(conn: Connection) -> Option<User> {
    print!("Enter searched email: ");
    let input: String = read!();
    select_user_by_email(&conn, &input)
}
