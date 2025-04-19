use super::structs::User;
use rusqlite::{params, Connection};

pub fn update_user_profile_picture(conn: &Connection, user: &User) -> Result<(), std::io::Error> {
    let query = "UPDATE User SET profile_picture=?1 WHERE email=?2";
    conn.execute(query, params![user.profile_picture, user.email])
        .expect("update_user_image query failed");
    println!("Successfully updated User {:?} in db", user.profile_picture);
    Ok(())
}

pub fn update_user_last_connection(conn: &Connection, user: &User) -> Result<(), std::io::Error> {
    let query = "UPDATE User SET last_connection=?1 WHERE email=?2";
    conn.execute(query, params![user.last_connection, user.email])
        .expect("update_user_last_connection query failed");
    println!("Successfully updated User {:?} in db", user.profile_picture);
    Ok(())
}
