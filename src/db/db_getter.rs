use rusqlite::{Connection, Result};
use super::structs::User;

pub fn get_user_usernames(conn: Connection) -> Result<Vec<(i32,String)>> {
    let mut format = conn.prepare("SELECT id, username, email, password_hash, password_salt, last_connection, account_creation, profile_picture FROM User")?;
    let user_iter = format.query_map([], |row| {
        Ok( User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: String::new(),
            password_hash: String::new(),
            password_salt: String::new(),
            last_connection: 0,
            account_creation: 0,
            profile_picture: None,
        })
    })?;
    
    let mut usernames = Vec::new();
    for user in user_iter {
        match user {
            Ok(user) => usernames.push((user.id,user.username)),
            Err(e) => return Err(e),
        }
    }

    Ok(usernames)
}
