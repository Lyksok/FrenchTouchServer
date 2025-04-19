use rusqlite::Connection;

pub fn user_exist_by_id(conn: &Connection, id: i64) -> bool {
    conn.query_row("SELECT id FROM User WHERE id=?1", [id], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}

pub fn user_exist_by_email(conn: &Connection, email: &str) -> bool {
    conn.query_row("SELECT id FROM User WHERE email=?1", [email], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}

pub fn artist_exist_by_id(conn: &Connection, id: i64) -> bool {
    conn.query_row("SELECT id FROM Artist WHERE id=?1", [id], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}

pub fn song_exist_by_id(conn: &Connection, id: i64) -> bool {
    conn.query_row("SELECT id FROM Song WHERE id=?1", [id], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}

pub fn album_exist_by_id(conn: &Connection, id: i64) -> bool {
    conn.query_row("SELECT id FROM Album WHERE id=?1", [id], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}

pub fn playlist_exist_by_id(conn: &Connection, id: i64) -> bool {
    conn.query_row("SELECT id FROM Playlist WHERE id=?1", [id], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}
