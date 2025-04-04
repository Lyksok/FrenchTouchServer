use super::structs::User;
use rusqlite::{Connection, params};
use text_io::read;

pub fn insert_user(conn: Connection, user: User) -> Result<(), std::io::Error> {
    let query = 
        "INSERT INTO User \
        (username,email,password_hash,password_salt,last_connection,creation_date,profile_picture) \
        VALUES (?1,?2,?3,?4,?5,?6,?7)";
    conn.execute(query,
        params![
            user.username,
            user.email,
            user.password_hash,
            user.password_salt,
            user.last_connection,
            user.creation_date,
            user.profile_picture
        ],).expect("insert_user query failed");
    Ok(println!("Successfully added User {:?} in db",user.username))
}

pub fn dev_insert_user(conn: Connection) -> Result<(), std::io::Error> {
    print!("Username: ");
    let username = read!();
    print!("Email: ");
    let email = read!();
    print!("Password Hash: ");
    let password_hash = read!();
    print!("Password Salt: ");
    let password_salt = read!();
    let user = User {
        id: -1,
        username: username,
        email: email,
        password_hash: password_hash,
        password_salt: password_salt,
        last_connection: 0,
        creation_date: 0,
        profile_picture: Some(String::new()),
    };

    insert_user(conn, user)
}
