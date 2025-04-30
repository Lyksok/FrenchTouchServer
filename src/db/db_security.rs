use openssl::base64::{decode_block, encode_block};
use openssl::rand::rand_bytes;
use pbkdf2::pbkdf2_hmac;
use rusqlite::Connection;
use sha2::Sha256;

use super::{db_select::select_authmap_by_auth_hash, structs::User};

pub fn has_permissions(conn: &Connection, user: &User, auth_hash: &str, p_level: i32) -> bool {
    match select_authmap_by_auth_hash(conn, auth_hash) {
        Some(authmap) => {
            if authmap.permission_level >= p_level && authmap.user_id == user.id {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn hash_password_with_salt(p: &str, s: Vec<u8>) -> (String, String) {
    let iterations = 600_000;
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
