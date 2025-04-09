use super::structs::User;
use rusqlite::{Connection, params};

pub fn update_user_profile_picture(
    conn: &Connection,
    user: &User,
) -> Result<(), std::io::Error> {
    let query = "UPDATE User SET profile_picture=?1 WHERE email=?2";
    conn.execute(query,
        params![
            user.profile_picture,
            user.email
        ],).expect("update_user_image query failed");
    Ok(println!("Successfully updated User {:?} in db",user.profile_picture))
}


