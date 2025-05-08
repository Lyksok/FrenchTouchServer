use openssl::base64::{decode_block, encode_block};
use openssl::rand::rand_bytes;
use pbkdf2::pbkdf2_hmac;
use rusqlite::Connection;
use sha2::Sha256;
use crate::db;

use crate::api::api_utils::{HistoryRequest, UserLikesAlbumRequest, UserLikesArtistRequest, UserLikesPlaylistRequest, UserLikesSongRequest, UserRequest};

use super::db_select::select_authmap_by_auth_hash;

pub fn has_permissions(conn: &Connection, auth_hash: &str, p_level: i32) -> bool {
    match select_authmap_by_auth_hash(conn, auth_hash) {
        Some(authmap) => {
            if authmap.permission_level >= p_level {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn has_exact_permissions(conn: &Connection, auth_hash: &str, p_level: i32) -> bool {
    match select_authmap_by_auth_hash(conn, auth_hash) {
        Some(authmap) => {
            if authmap.permission_level == p_level {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn has_permissions_user(conn: &Connection, user_req: &UserRequest) -> bool {
    match db::db_select::select_authmap_by_user_id(conn, user_req.obj.id) {
        Some(authmap) => authmap.auth_hash == user_req.auth_hash,
        None => false,
    }
}

pub fn has_permissions_user_likes_song(conn: &Connection, user_req: &UserLikesSongRequest) -> bool {
    match db::db_select::select_authmap_by_user_id(conn, user_req.obj.user_id) {
        Some(authmap) => authmap.auth_hash == user_req.auth_hash,
        None => false,
    }
}

pub fn has_permissions_user_likes_album(conn: &Connection, user_req: &UserLikesAlbumRequest) -> bool {
    match db::db_select::select_authmap_by_user_id(conn, user_req.obj.user_id) {
        Some(authmap) => authmap.auth_hash == user_req.auth_hash,
        None => false,
    }
}

pub fn has_permissions_user_likes_playlist(conn: &Connection, user_req: &UserLikesPlaylistRequest) -> bool {
    match db::db_select::select_authmap_by_user_id(conn, user_req.obj.user_id) {
        Some(authmap) => authmap.auth_hash == user_req.auth_hash,
        None => false,
    }
}

pub fn has_permissions_user_likes_artist(conn: &Connection, user_req: &UserLikesArtistRequest) -> bool {
    match db::db_select::select_authmap_by_user_id(conn, user_req.obj.user_id) {
        Some(authmap) => authmap.auth_hash == user_req.auth_hash,
        None => false,
    }
}

pub fn has_permissions_user_history(conn: &Connection, user_req: &HistoryRequest) -> bool {
    match db::db_select::select_authmap_by_user_id(conn, user_req.obj.user_id) {
        Some(authmap) => authmap.auth_hash == user_req.auth_hash,
        None => false,
    }
}

pub fn hash_password_with_salt(p: &str, s: Vec<u8>) -> (String, String) {
    let iterations = 200_000;
    let mut buf = vec![0u8; 32];
    pbkdf2_hmac::<Sha256>(p.as_bytes(), &s, iterations, &mut buf);

    let salt = encode_block(&s);
    let hash = encode_block(&buf);

    (hash, salt)
}

pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 32];
    rand_bytes(&mut salt).expect("Failed to generate random bytes");
    salt
}

pub fn new_hash_password(p: &str) -> (String, String) {
    hash_password_with_salt(p, generate_salt())
}

pub fn check_password(p: &str, s: &str, h: &str) -> bool {
    h == hash_password_with_salt(p, decode_block(s).unwrap()).0
}
