use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub last_connection: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub last_connection: i32,
    pub creation_date: i32,
    pub profile_picture: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: i64,
    pub user_id: i64,
    pub nb_of_streams: i32,
    pub biographie: Option<String>,
    pub url: Option<String>,
    pub verified: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    pub id: i64,
    pub user_id: i64,
    pub verified: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: i64,
    pub name: String,
    pub song_file: Option<String>,
    pub nb_of_streams: i32,
    pub cover_image: Option<String>,
    pub duration: i32,
    pub creation_date: i32,
    pub artist_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover: Option<String>,
    pub creation_date: i32,
    pub artist_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: i64,
    pub title: String,
    pub cover: Option<String>,
    pub creation_date: i32,
    pub user_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLikesSong {
    pub user_id: i64,
    pub song_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLikesAlbum {
    user_id: i64,
    album_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLikesPlaylist {
    user_id: i64,
    playlist_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListensSong {
    user_id: i64,
    song_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAlbum {
    song_id: i64,
    album_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongPlaylist {
    song_id: i64,
    playlist_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    user_id: i64,
    song_id: i64,
    time: i32,
}
