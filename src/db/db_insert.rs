use text_io::read;

use super::structs::{
    Admin, Album, Artist, ArtistRequest, AuthMap, Collaborator, CollaboratorRequest, Credentials, History, Playlist, RequestToAdmin, RequestToArtist, RequestToCollaborator, Song, SongAlbum, SongPlaylist, User, UserLikesAlbum, UserLikesArtist, UserLikesPlaylist, UserLikesSong
};
use crate::db::{
    self, db_insert, db_select::{self, select_user_by_email}
};
use rusqlite::{Connection, params};

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
        (username,email,last_connection,creation_date,profile_picture) \
        VALUES (?1,?2,?3,?4,?5)";
    match conn.execute(
        query,
        params![
            user.username,
            user.email,
            user.last_connection,
            user.creation_date,
            user.profile_picture,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_credentials(conn: &Connection, credentials: &Credentials) -> Option<i64> {
    let query = "INSERT INTO Credentials \
        (user_id,password_hash,password_salt) \
        VALUES (?1,?2,?3)";
    match conn.execute(
        query,
        params![
            credentials.user_id,
            credentials.password_hash,
            credentials.password_salt,
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

pub fn insert_user_likes_artist(
    conn: &Connection,
    user_likes_artist: &UserLikesArtist,
) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, user_likes_artist.user_id)
        || !db::db_exist::artist_exist_by_id(conn, user_likes_artist.artist_id)
    {
        return None;
    }
    let query = "INSERT INTO UserLikesArtist \
        (user_id,artist_id) \
        VALUES (?1,?2)";
    match conn.execute(
        query,
        params![user_likes_artist.user_id, user_likes_artist.artist_id,],
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
        || db::db_exist::authmap_exist_by_hash(conn, &auth_map.auth_hash)
    {
        return None;
    }
    let query = "INSERT INTO AuthMap \
        (user_id,auth_hash,permission_level) \
        VALUES (?1,?2,?3)";
    match conn.execute(
        query,
        params![
            auth_map.user_id,
            auth_map.auth_hash,
            auth_map.permission_level,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_artist_request(
    conn: &Connection,
    artist_req: &ArtistRequest,
) -> Option<i64> {
    if !db::db_exist::artist_exist_by_id(conn, artist_req.artist_id) {
        return None;
    }
    let query = "INSERT INTO ArtistRequest \
        (artist_id,song_title,song_creation_date,song_file,song_cover,album_id,album_name,album_creation_date,album_cover) \
        VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)";
    match conn.execute(
        query,
        params![
            artist_req.artist_id,
            artist_req.song_title,
            artist_req.song_creation_date,
            artist_req.song_file,
            artist_req.song_cover,
            artist_req.album_id,
            artist_req.album_name,
            artist_req.album_creation_date,
            artist_req.album_cover,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_collaboration_request(
    conn: &Connection,
    collab_req: &CollaboratorRequest,
) -> Option<i64> {
    if !db::db_exist::collaborator_exist_by_id(conn, collab_req.collaborator_id) {
        return None;
    }
    let query = "INSERT INTO CollaboratorRequest \
        (collaborator_id,song_title,song_creation_date,song_file,song_cover,artist_id,artist_name,artist_biography,artist_profile_picture,album_id,album_name,album_creation_date,album_cover) \
        VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)";
    match conn.execute(
        query,
        params![
            collab_req.collaborator_id,
            collab_req.song_title,
            collab_req.song_creation_date,
            collab_req.song_file,
            collab_req.song_cover,
            collab_req.artist_id,
            collab_req.artist_name,
            collab_req.artist_biography,
            collab_req.artist_profile_picture,
            collab_req.album_id,
            collab_req.album_name,
            collab_req.album_creation_date,
            collab_req.album_cover,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_request_to_artist(
    conn: &Connection,
    req: &RequestToArtist,
) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, req.user_id) {
        return None;
    }
    let query = "INSERT INTO RequestToArtist \
        (user_id) \
        VALUES (?)";
    match conn.execute(
        query,
        params![
            req.user_id,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_request_to_collaborator(
    conn: &Connection,
    req: &RequestToCollaborator,
) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, req.user_id) {
        return None;
    }
    let query = "INSERT INTO RequestToCollaborator \
        (user_id) \
        VALUES (?)";
    match conn.execute(
        query,
        params![
            req.user_id,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

pub fn insert_request_to_admin(
    conn: &Connection,
    req: &RequestToAdmin,
) -> Option<i64> {
    if !db::db_exist::user_exist_by_id(conn, req.user_id) {
        return None;
    }
    let query = "INSERT INTO RequestToAdmin \
        (user_id) \
        VALUES (?)";
    match conn.execute(
        query,
        params![
            req.user_id,
        ],
    ) {
        Ok(_) => Some(conn.last_insert_rowid()),
        Err(_) => None,
    }
}

// ------------------------------------------ DEV ZONE

pub fn dev_insert_admin(conn: &Connection) -> () {
    print!("Enter user email: ");
    let input: String = read!();
    let user = match select_user_by_email(&conn, &input) {
        Some(user) => user,
        None => return (),
    };
    let admin = Admin {
        id: -1,
        user_id: user.id,
    };
    let _ = insert_admin(&conn, &admin);
    let _ = db::db_update::update_authmap(&conn, user.id, 3);
}

pub fn dev_insert_collaborator(conn: &Connection) -> () {
    print!("Enter user email: ");
    let input: String = read!();
    let user = match select_user_by_email(&conn, &input) {
        Some(user) => user,
        None => return (),
    };
    let collaborator = Collaborator{
        id: -1,
        user_id: user.id,
        verified: false,
    };
    let _ = insert_collaborator(&conn, &collaborator);
}

pub fn dev_insert_song_album(conn: &Connection) -> bool {
    println!("Artists {:?}",db_select::select_artist_all(&conn));
    print!("Enter artist id: ");
    let input: i64 = read!();
    println!("Songs {:?}",db_select::select_song_by_artist_id(&conn, input));
    print!("Enter song id: ");
    let input2: i64 = read!();
    println!("Albums {:?}",db_select::select_album_by_artist_id(&conn, input));
    print!("Enter album id: ");
    let input3: i64 = read!();
    db_insert::insert_song_album(&conn, &SongAlbum { song_id: input2, album_id: input3}).is_some()
}
