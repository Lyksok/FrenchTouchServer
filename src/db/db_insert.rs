use super::structs::{
    Admin, Album, Artist, AuthMap, Collaborator, History, Playlist, Song, SongAlbum, SongPlaylist,
    User, UserLikesAlbum, UserLikesPlaylist, UserLikesSong,
};
use crate::db;
use rusqlite::{params, Connection};
use text_io::read;
pub fn insert_admin(conn: &Connection, admin: &Admin) -> Option<i64> {
    let query = "INSERT INTO Admin \
        (user_id) \
        VALUES (?1)";
    match conn.execute(query, params![admin.user_id,]) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_user(conn: &Connection, user: &User) -> Option<i64> {
    let query = "INSERT INTO User \
        (username,email,password_hash,password_salt,last_connection,creation_date) \
        VALUES (?1,?2,?3,?4,?5,?6)";
    match conn.execute(
        query,
        params![
            user.username,
            user.email,
            user.password_hash,
            user.password_salt,
            user.last_connection,
            user.creation_date
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_artist(conn: &Connection, artist: &Artist) -> Option<i64> {
    if artist.user_id == -1 {
        return None;
    }
    let query = "INSERT INTO Artist \
        (user_id,nb_of_streams,biography,url,verified) \
        VALUES (?1,?2,?3,?4,?5)";
    match conn.execute(
        query,
        params![
            artist.user_id,
            artist.nb_of_streams,
            artist.biography,
            artist.url,
            artist.verified,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_collaborator(conn: &Connection, collaborator: &Collaborator) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, collaborator.user_id) {
        return None;
    }
    let query = "INSERT INTO Collaborator \
        (user_id,verified) \
        VALUES (?1,?2)";
    match conn.execute(query, params![collaborator.user_id, 0,]) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_song(conn: &Connection, song: &Song) -> Option<i64> {
    let query = "INSERT INTO Song \
        (title,song_file,cover_image,nb_of_streams,duration,creation_date,artist_id) \
        VALUES (?1,?2,?3,?4,?5,?6,?7)";
    match conn.execute(
        query,
        params![
            song.title,
            song.song_file,
            song.cover_image,
            song.nb_of_streams,
            song.duration,
            song.creation_date,
            song.artist_id,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_album(conn: &Connection, album: &Album) -> Option<i64> {
    if !db::db_exist::artist_exist_by_id(conn, album.artist_id) {
        return None;
    }
    let query = "INSERT INTO Album \
        (title,cover_image,nb_of_streams,creation_date,artist_id) \
        VALUES (?1,?2,?3,?4,?5)";
    match conn.execute(
        query,
        params![
            album.title,
            album.cover_image,
            album.nb_of_streams,
            album.creation_date,
            album.artist_id,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_playlist(conn: &Connection, playlist: &Playlist) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, playlist.user_id) {
        return None;
    }
    let query = "INSERT INTO Playlist \
        (title,cover_image,nb_of_streams,creation_date,user_id) \
        VALUES (?1,?2,?3,?4,?5)";
    match conn.execute(
        query,
        params![
            playlist.title,
            playlist.cover_image,
            playlist.nb_of_streams,
            playlist.creation_date,
            playlist.user_id,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_user_likes_song(conn: &Connection, user_likes_song: &UserLikesSong) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, user_likes_song.user_id)
        || !db::db_exist::song_exist_by_id(conn, user_likes_song.song_id)
    {
        return None;
    }
    let query = "INSERT INTO UserLikesSong \
        (user_id,song_id) \
        VALUES (?1,?2)";
    match conn.execute(
        query,
        params![user_likes_song.user_id, user_likes_song.song_id,],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_user_likes_album(
    conn: &Connection,
    user_likes_album: &UserLikesAlbum,
) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, user_likes_album.user_id)
        || !db::db_exist::album_exist_by_id(conn, user_likes_album.album_id)
    {
        return None;
    }
    let query = "INSERT INTO UserLikesAlbum \
        (user_id,album_id) \
        VALUES (?1,?2)";
    match conn.execute(
        query,
        params![user_likes_album.user_id, user_likes_album.album_id,],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_user_likes_playlist(
    conn: &Connection,
    user_likes_playlist: &UserLikesPlaylist,
) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, user_likes_playlist.user_id)
        || !db::db_exist::playlist_exist_by_id(conn, user_likes_playlist.playlist_id)
    {
        return None;
    }
    let query = "INSERT INTO UserLikesPlaylist \
        (user_id,playlist_id) \
        VALUES (?1,?2)";
    match conn.execute(
        query,
        params![user_likes_playlist.user_id, user_likes_playlist.playlist_id,],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_song_album(conn: &Connection, song_album: &SongAlbum) -> Option<i64> {
    if !db::db_exist::song_exist_by_id(conn, song_album.song_id)
        || !db::db_exist::album_exist_by_id(conn, song_album.album_id)
    {
        return None;
    }
    let query = "INSERT INTO SongAlbum \
        (song_id,album_id) \
        VALUES (?1,?2)";
    match conn.execute(query, params![song_album.song_id, song_album.album_id,]) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_song_playlist(conn: &Connection, song_playlist: &SongPlaylist) -> Option<i64> {
    if !db::db_exist::song_exist_by_id(conn, song_playlist.song_id)
        || !db::db_exist::playlist_exist_by_id(conn, song_playlist.playlist_id)
    {
        return None;
    }
    let query = "INSERT INTO SongPlaylist \
        (song_id,playlist_id) \
        VALUES (?1,?2)";
    match conn.execute(
        query,
        params![song_playlist.song_id, song_playlist.playlist_id,],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_history(conn: &Connection, history: &History) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, history.user_id)
        || !db::db_exist::song_exist_by_id(conn, history.song_id)
    {
        return None;
    }
    let query = "INSERT INTO History \
        (user_id,song_id,time) \
        VALUES (?1,?2,?3)";
    match conn.execute(
        query,
        params![history.user_id, history.song_id, history.time],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_authmap(conn: &Connection, auth_map: &AuthMap) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, auth_map.user_id)
        || !db::db_exist::authmap_exist_by_hash(conn, &auth_map.auth_hash)
    {
        return None;
    }
    let query = "INSERT INTO AuthMap \
        (user_id,auth_hash,permission_level,expire_date) \
        VALUES (?1,?2,?3,?4)";
    match conn.execute(
        query,
        params![
            auth_map.user_id,
            auth_map.auth_hash,
            auth_map.permission_level,
            auth_map.expiration_date
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

// =================================================================== DEV ZONE

pub fn dev_insert_user(conn: Connection) -> bool {
    print!("Username: ");
    let username = read!();
    print!("Email: ");
    let email = read!();
    print!("Password Hash: ");
    let password_hash = read!();
    print!("Password Salt: ");
    let password_salt = read!();
    let user = User {
        id: -1,
        username,
        email,
        password_hash,
        password_salt,
        last_connection: 0,
        creation_date: 0,
        profile_picture: Some(String::new()),
    };
    let res = insert_user(&conn, &user);
    if res.is_some() {
        println!("User inserted !");
    } else {
        println!("Could not insert user");
    }
    res.is_some()
}
