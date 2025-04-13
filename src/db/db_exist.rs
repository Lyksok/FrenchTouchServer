use rusqlite::{params, Connection};

pub fn user_exist_by_id(conn: &Connection, id: i32) -> bool {
    match conn.prepare("SELECT id FROM User WHERE id=?1") {
        Ok(mut format) => {
            let iter = format.query_map(params![id], |row| {
                let res: i32 = row.get(0)?;
                Ok(res)
            });
            iter.iter().count() != 0
        }
        Err(_) => false,
    }
}

pub fn user_exist_by_email(conn: &Connection, email: &str) -> bool {
    match conn.prepare("SELECT id FROM User WHERE email=?1") {
        Ok(mut format) => {
            let iter = format.query_map(params![email], |row| {
                let res: i32 = row.get(0)?;
                Ok(res)
            });

            iter.iter().count() != 0
        }
        Err(_) => false,
    }
}
