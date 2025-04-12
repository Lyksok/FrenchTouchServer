use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub last_connection: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
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
    pub id: i32,
    pub user_id: i32,
    pub nb_of_streams: i32,
    pub biographie: Option<String>,
    pub url: Option<String>,
    pub verified: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    pub id: i32,
    pub user_id: i32,
    pub verified: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: i32,
    pub name: String,
    pub song_file: Option<String>,
    pub nb_of_streams: i32,
    pub cover_image: Option<String>,
    pub duration: i32,
    pub creation_date: i32,
    pub artist_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    id: i32,
    title: String,
    cover: Option<String>,
    creation_date: i32,
    artist_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    id: i32,
    title: String,
    cover: Option<String>,
    creation_date: i32,
    user_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLikesSong {
    user_id: i32,
    song_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLikesAlbum {
    user_id: i32,
    album_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLikesPlaylist {
    user_id: i32,
    playlist_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListensSong {
    user_id: i32,
    song_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAlbum {
    song_id: i32,
    album_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongPlaylist {
    song_id: i32,
    playlist_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    user_id: i32,
    song_id: i32,
    time: i32,
}
