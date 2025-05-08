use rusqlite::{params, Connection};

use super::structs::{UserLikesAlbum, UserLikesArtist, UserLikesPlaylist, UserLikesSong};

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

pub fn delete_collaborator_request_by_id(conn: &Connection, id: i64) -> bool {
    let query = "DELETE FROM CollaboratorRequest WHERE CollaboratorRequest.id=?";
    match conn.execute(query, params![id]) {
        Ok(_) => {
            println!("[DELETE] Deleted CollaboratorRequest : {}", id);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete CollaboratorRequest : {}", e);
            false
        }
    }
}

pub fn delete_artist_request_by_id(conn: &Connection, id: i64) -> bool {
    let query = "DELETE FROM ArtistRequest WHERE ArtistRequest.id=?";
    match conn.execute(query, params![id]) {
        Ok(_) => {
            println!("[DELETE] Deleted ArtistRequest : {}", id);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete ArtistRequest : {}", e);
            false
        }
    }
}

pub fn delete_request_to_collaborator_by_user_id(conn: &Connection, user_id: i64) -> bool {
    let query = "DELETE FROM RequestToCollaborator WHERE RequestToCollaborator.user_id=?";
    match conn.execute(query, params![user_id]) {
        Ok(_) => {
            println!("[DELETE] Deleted RequestToCollaborator : {}", user_id);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete RequestToCollaborator : {}", e);
            false
        }
    }
}

pub fn delete_request_to_artist_by_user_id(conn: &Connection, user_id: i64) -> bool {
    let query = "DELETE FROM RequestToArtist WHERE RequestToArtist.user_id=?";
    match conn.execute(query, params![user_id]) {
        Ok(_) => {
            println!("[DELETE] Deleted RequestToArtist : {}", user_id);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete RequestToArtist : {}", e);
            false
        }
    }
}

pub fn delete_request_to_admin_by_user_id(conn: &Connection, user_id: i64) -> bool {
    let query = "DELETE FROM RequestToAdmin WHERE RequestToAdmin.user_id=?";
    match conn.execute(query, params![user_id]) {
        Ok(_) => {
            println!("[DELETE] Deleted RequestToAdmin : {}", user_id);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete RequestToAdmin : {}", e);
            false
        }
    }
}

pub fn delete_user_likes_song(conn: &Connection, uls: &UserLikesSong) -> bool {
    let query = "DELETE FROM UserLikesSong WHERE UserLikesSong.user_id=?1 AND UserLikesSong.song_id=?2";
    match conn.execute(query, params![uls.user_id, uls.song_id]) {
        Ok(_) => {
            println!("[DELETE] Deleted UserLikesSong : {:?}", uls);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete UserLikesSong : {}", e);
            false
        }
    }
}

pub fn delete_user_likes_album(conn: &Connection, uls: &UserLikesAlbum) -> bool {
    let query = "DELETE FROM UserLikesAlbum WHERE UserLikesAlbum.user_id=?1 AND UserLikesAlbum.album_id=?2";
    match conn.execute(query, params![uls.user_id, uls.album_id]) {
        Ok(_) => {
            println!("[DELETE] Deleted UserLikesAlbum : {:?}", uls);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete UserLikesAlbum : {}", e);
            false
        }
    }
}

pub fn delete_user_likes_playlist(conn: &Connection, uls: &UserLikesPlaylist) -> bool {
    let query = "DELETE FROM UserLikesPlaylist WHERE UserLikesPlaylist.user_id=?1 AND UserLikesPlaylist.playlist_id=?2";
    match conn.execute(query, params![uls.user_id, uls.playlist_id]) {
        Ok(_) => {
            println!("[DELETE] Deleted UserLikesPlaylist : {:?}", uls);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete UserLikesPlaylist : {}", e);
            false
        }
    }
}

pub fn delete_user_likes_artist(conn: &Connection, uls: &UserLikesArtist) -> bool {
    let query = "DELETE FROM UserLikesArtist WHERE UserLikesArtist.user_id=?1 AND UserLikesArtist.artist_id=?2";
    match conn.execute(query, params![uls.user_id, uls.artist_id]) {
        Ok(_) => {
            println!("[DELETE] Deleted UserLikesArtist : {:?}", uls);
            true
        }
        Err(e) => {
            println!("[DELETE ERROR] Could not delete UserLikesArtist : {}", e);
            false
        }
    }
}