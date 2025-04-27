use rusqlite::{params, Connection};

pub fn delete_authmap_by_auth_hash(conn: &Connection, auth_hash: &str) -> bool {
    let query = "DELETE FROM AuthMap WHERE AuthMap.auth_hash LIKE ?1";
    match conn.execute(query, params![auth_hash]) {
        Ok(_) => {
            println!("[DELETE] Deleted auth map hash : {}", auth_hash);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete auth map : {}", e);
            false
        }
    }
}
