use super::structs::{AuthMap, CollaboratorRequest, Credentials, User};
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

pub fn update_collaborator_request_all(conn: &Connection, collab_req: &CollaboratorRequest) -> Result<(), String> {
    let query = "UPDATE CollaboratorRequest SET collaborator_id=?2,song_title=?3,song_creation_date=?4,song_file=?5,song_cover=?6,artist_name=?7,artist_biography=?8,artist_profile_picture=?8 WHERE id=?1";
    match conn.execute(query, params![
            collab_req.id,
            collab_req.collaborator_id,
            collab_req.song_title,
            collab_req.song_creation_date,
            collab_req.song_file,
            collab_req.song_cover,
            collab_req.artist_name,
            collab_req.artist_biography,
            collab_req.artist_profile_picture,
            ]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}",e)),
    }
}

pub fn update_authmap(conn: &Connection, user_id: i64, permission_level: i32) -> Result<(), String> {
    let query = "UPDATE AuthMap SET permission_level=?2 WHERE user_id=?1";
    match conn.execute(query, params![user_id, permission_level]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}",e)),
    }
}

pub fn update_credentials(conn: &Connection, cred: &Credentials) -> Result<(), String> {
    let query = "UPDATE Credentials SET password_hash=?2,password_salt=?3 WHERE user_id=?1";
    match conn.execute(query, params![cred.user_id, cred.password_hash,cred.password_salt]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{}",e)),
    }
}
