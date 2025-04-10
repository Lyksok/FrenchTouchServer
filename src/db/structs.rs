use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone)]
pub struct Admin {
    id: i32,
    email: String,
    username: String,
    last_connection: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Song {
    id: i32,
    name: String,
    song_file: Option<String>,
    nb_of_streams: i32,
    cover_image: Option<String>,
    duration: i32,
    creation_date: i32,
    artist_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Artist {
    id: i32,
    user_id: i32,
    nb_of_streams: i32,
    biographie: Option<String>,
    url: Option<String>,
    verified: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Album {
    id: i32,
    title: String,
    cover: Option<String>,
    creation_date: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Playlist {
    id: i32,
    title: String,
    cover: Option<String>,
    creation_date: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UserLikesSong {
    user_id: i32,
    song_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UserLikesAlbum {
    user_id: i32,
    album_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UserLikesPlaylist {
    user_id: i32,
    playlist_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UserListensSong {
    user_id: i32,
    song_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SongAlbum {
    song_id: i32,
    album_id: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SongPlaylist {
    song_id: i32,
    playlist_id: i32,
}
