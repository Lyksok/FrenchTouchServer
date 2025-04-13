use super::{
    db_exist::user_exist_by_id,
    structs::{Collaborator, Song, User},
};
use rusqlite::{params, Connection};
use text_io::read;

pub fn insert_user(conn: &Connection, user: &User) -> bool {
    let query = "INSERT INTO User \
        (username,email,password_hash,password_salt,last_connection,creation_date) \
        VALUES (?1,?2,?3,?4,?5,?6)";
    conn.execute(
        query,
        params![
            user.username,
            user.email,
            user.password_hash,
            user.password_salt,
            user.last_connection,
            user.creation_date
        ],
    )
    .is_ok()
}

pub fn insert_song(conn: &Connection, song: &Song) -> bool {
    let query = "INSERT INTO Song \
        (file,name,length,nb_of_streams,cover,creation_date,artist_id) \
        VALUES (?1,?2,?3,?4,?5,?6,?7)";
    conn.execute(
        query,
        params![
            song.song_file,
            song.name,
            song.duration,
            song.nb_of_streams,
            song.cover_image,
            song.creation_date,
            song.artist_id
        ],
    )
    .is_ok()
}

pub fn insert_collaborator(conn: &Connection, user: &User, collaborator: &Collaborator) -> bool {
    if !user_exist_by_id(conn, user.id) {
        return false;
    }
    let query = "INSERT INTO Collaborator \
        (user_id,verified) \
        VALUES (?1,?2)";
    conn.execute(query, params![collaborator.user_id, 0,])
        .is_ok()
}

pub fn dev_insert_user(conn: Connection) -> bool {
    print!("Username: ");
    let username = read!();
    print!("Email: ");
    let email = read!();
    print!("Password Hash: ");
    let password_hash = read!();
    print!("Password Salt: ");
    let password_salt = read!();
    let user = User {
        id: -1,
        username,
        email,
        password_hash,
        password_salt,
        last_connection: 0,
        creation_date: 0,
        profile_picture: Some(String::new()),
    };
    let res = insert_user(&conn, &user);
    if res {
        println!("User inserted !");
    } else {
        println!("Could not insert user");
    }
    res
}
