use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub last_connection: i32,
    pub creation_date: i32,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub user_id: i64,
    pub password_hash: String,
    pub password_salt: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: i64,
    pub user_id: i64,
    pub nb_of_streams: i32,
    pub biography: Option<String>,
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
    pub title: String,
    pub song_file: Option<String>,
    pub cover_image: Option<String>,
    pub nb_of_streams: i32,
    pub duration: i32,
    pub creation_date: i32,
    pub artist_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover_image: Option<String>,
    pub nb_of_streams: i32,
    pub creation_date: i32,
    pub artist_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: i64,
    pub title: String,
    pub cover_image: Option<String>,
    pub nb_of_streams: i32,
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
    pub user_id: i64,
    pub album_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLikesPlaylist {
    pub user_id: i64,
    pub playlist_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAlbum {
    pub song_id: i64,
    pub album_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongPlaylist {
    pub song_id: i64,
    pub playlist_id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub user_id: i64,
    pub song_id: i64,
    pub time: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRequest {
    pub id: i64,
    pub artist_id: i64,
    pub song_title: String,
    pub song_creation_date: i32,
    pub song_file: String,
    pub song_cover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorRequest {
    pub id: i64,
    pub collaborator_id: i64,
    pub song_title: String,
    pub song_creation_date: i32,
    pub song_file: String,
    pub song_cover: Option<String>,
    pub artist_name: String,
    pub artist_biography: Option<String>,
    pub artist_profile_picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthMap {
    pub user_id: i64,
    pub auth_hash: String,
    // 0:User 1:Artist 2:Collaborator 3:Admin
    pub permission_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestToArtist {
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestToCollaborator {
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestToAdmin {
    pub user_id: i64,
}