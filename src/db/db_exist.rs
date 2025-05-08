use rusqlite::Connection;

use super::structs::{UserLikesAlbum, UserLikesPlaylist, UserLikesSong};

pub fn admin_exist_by_user_id(conn: &Connection, user_id: i64) -> bool {
    conn.query_row("SELECT id FROM Admin WHERE user_id=?1", [user_id], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}

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

pub fn artist_exist_by_user_id(conn: &Connection, user_id: i64) -> bool {
    conn.query_row("SELECT id FROM Artist WHERE user_id=?1", [user_id], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}

pub fn collaborator_exist_by_id(conn: &Connection, collaborator_id: i64) -> bool {
    conn.query_row("SELECT id FROM Collaborator WHERE id=?1", [collaborator_id], |row|{
        let res:i64=row.get(0)?;
        Ok(res)
    })
    .is_ok()
}


pub fn collaborator_exist_by_user_id(conn: &Connection, user_id: i64) -> bool {
    conn.query_row(
        "SELECT id FROM Collaborator WHERE user_id=?1",
        [user_id],
        |row| {
            let res: i64 = row.get(0)?;
            Ok(res)
        },
    )
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

pub fn user_likes_song_exist(conn: &Connection, like: &UserLikesSong)->bool{
    conn.query_row("SELECT user_id FROM UserLikesSong WHERE user_id=?1 AND song_id=?2", [like.user_id, like.song_id], |row|{
        let res: i64 = row.get(0)?;
        Ok(res)
    }).is_ok()
}

pub fn user_likes_album_exist(conn: &Connection, like: &UserLikesAlbum)->bool{
    conn.query_row("SELECT user_id FROM UserLikesAlbum WHERE user_id=?1 AND album_id=?2", [like.user_id, like.album_id], |row|{
        let res: i64 = row.get(0)?;
        Ok(res)
    }).is_ok()
}

pub fn user_likes_playlist_exist(conn: &Connection, like: &UserLikesPlaylist)->bool{
    conn.query_row("SELECT user_id FROM UserLikesPlaylist WHERE user_id=?1 AND playlist_id=?2", [like.user_id, like.playlist_id], |row|{
        let res: i64 = row.get(0)?;
        Ok(res)
    }).is_ok()
}

pub fn authmap_exist_by_id(conn: &Connection, id: i64) -> bool {
    conn.query_row("SELECT id FROM AuthMap WHERE id=?1", [id], |row| {
        let res: i64 = row.get(0)?;
        Ok(res)
    })
    .is_ok()
}

pub fn authmap_exist_by_user_id(conn: &Connection, user_id: i64) -> bool {
    conn.query_row(
        "SELECT id FROM AuthMap WHERE user_id=?1",
        [user_id],
        |row| {
            let res: i64 = row.get(0)?;
            Ok(res)
        },
    )
    .is_ok()
}

pub fn authmap_exist_by_hash(conn: &Connection, auth_hash: &str) -> bool {
    conn.query_row(
        "SELECT id FROM AuthMap WHERE auth_hash LIKE ?1",
        [auth_hash],
        |row| {
            let res: i64 = row.get(0)?;
            Ok(res)
        },
    )
    .is_ok()
}

