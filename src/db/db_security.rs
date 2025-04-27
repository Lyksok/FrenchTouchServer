use openssl::base64::{decode_block, encode_block};
use openssl::rand::rand_bytes;
use pbkdf2::pbkdf2_hmac;
use rusqlite::Connection;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{
    db_delete::delete_authmap_by_auth_hash, db_select::select_authmap_by_auth_hash, structs::User,
};

pub fn is_valid_expiration_date(conn: &Connection, auth_hash: &str) -> bool {
    match select_authmap_by_auth_hash(conn, auth_hash) {
        Some(authmap) => {
            if authmap.expiration_date
                > SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i32
            {
                true
            } else {
                delete_authmap_by_auth_hash(conn, auth_hash);
                false
            }
        }
        _ => false,
    }
}

pub fn has_permissions(conn: &Connection, user: &User, auth_hash: &str, p_level: i32) -> bool {
    match select_authmap_by_auth_hash(conn, auth_hash) {
        Some(authmap) => {
            if authmap.permission_level >= p_level
                && is_valid_expiration_date(conn, auth_hash)
                && authmap.user_id == user.id
            {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn hash_password(p: &str, s: &str) -> Vec<String> {
    let iterations = 600_000;
    let mut buf = vec![0u8; 32];
    let salt = decode_block(s).unwrap();
    pbkdf2_hmac::<Sha256>(p.as_bytes(), &salt, iterations, &mut buf);

    let mut res = vec![];
    let salt = String::from("thbYFkTnABGkU/ma2dtkuA==");
    let hash = encode_block(&buf);

    res.push(hash);
    res.push(salt);
    res
}

pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 32];
    rand_bytes(&mut salt).expect("Failed to generate random bytes");
    salt
}
