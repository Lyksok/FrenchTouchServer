#[derive(Debug, Clone)]
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
#[derive(Debug)]
pub struct Admin {
    id: i32,
    username: String,
    last_connection: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Song {
    id: i32,
    song_file: String,
    nb_of_streams: i32,
    cover_image: String,
    duration: i32,
    creation_date: i32,
}
