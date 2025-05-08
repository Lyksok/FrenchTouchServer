use chrono::Local;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::db::structs::{
    Admin, Album, Artist, ArtistRequest, Collaborator, CollaboratorRequest, History, Playlist, Song, SongAlbum, SongPlaylist, User, UserLikesAlbum, UserLikesArtist, UserLikesPlaylist, UserLikesSong
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthHash {
    pub auth_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminRequest {
    pub auth_hash: String,
    pub obj: Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub auth_hash: String,
    pub obj: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistObjRequest {
    pub auth_hash: String,
    pub obj: Artist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollaboratorObjRequest {
    pub auth_hash: String,
    pub obj: Collaborator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongRequest {
    pub auth_hash: String,
    pub obj: Song,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumRequest {
    pub auth_hash: String,
    pub obj: Album,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistRequest {
    pub auth_hash: String,
    pub obj: Playlist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLikesSongRequest {
    pub auth_hash: String,
    pub obj: UserLikesSong,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLikesAlbumRequest {
    pub auth_hash: String,
    pub obj: UserLikesAlbum,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLikesPlaylistRequest {
    pub auth_hash: String,
    pub obj: UserLikesPlaylist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLikesArtistRequest {
    pub auth_hash: String,
    pub obj: UserLikesArtist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongAlbumRequest {
    pub auth_hash: String,
    pub obj: SongAlbum,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongPlaylistRequest {
    pub auth_hash: String,
    pub obj: SongPlaylist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryRequest {
    pub auth_hash: String,
    pub obj: History,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistRequestRequest {
    pub auth_hash: String,
    pub obj: ArtistRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollaboratorRequestRequest {
    pub auth_hash: String,
    pub obj: CollaboratorRequest,
}

pub fn print_log<T: Debug>(log_type: &str, entity_type: &str, entity: &T) {
    println!(
        "{} [{}] {}: {:?}",
        Local::now().format("%d/%m/%Y %H:%M"),
        log_type,
        entity_type,
        entity
    );
}
