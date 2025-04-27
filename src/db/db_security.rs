use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{db_select::select_authmap_by_auth_hash, structs::User};

pub fn has_permissions(conn: &Connection, user: &User, auth_hash: &str, p_level: i32) -> bool {
    match select_authmap_by_auth_hash(conn, auth_hash) {
        Some(authmap) => {
            if authmap.permission_level >= p_level
                && authmap.expiration_date
                    > SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i32
                && authmap.user_id == user.id
            {
                return true;
            }
            false
        }
        _ => false,
    }
}
