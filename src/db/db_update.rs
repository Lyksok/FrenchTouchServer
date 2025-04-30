use super::structs::{AuthMap, User};
use rusqlite::{params, Connection};

pub fn update_user_profile_picture(conn: &Connection, user: &User) -> Result<(), String> {
    let query = "UPDATE User SET profile_picture=?1 WHERE email=?2";
    match conn.execute(query, params![user.profile_picture, user.email]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}",e)),
    }
}

pub fn update_user_last_connection(conn: &Connection, user: &User) -> Result<(), String>  {
    let query = "UPDATE User SET last_connection=?1 WHERE email=?2";
    match conn.execute(query, params![user.last_connection, user.email]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}",e)),
    }
}

pub fn update_authmap_hash(conn: &Connection, auth_map: &AuthMap) -> Result<(), String>  {
    let query = "UPDATE AuthMap SET auth_hash=?1 WHERE user_id=?2";
    match conn.execute(query, params![auth_map.auth_hash, auth_map.user_id]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}",e)),
    }
}
