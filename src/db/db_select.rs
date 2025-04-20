use super::{
    db_exist::user_exist_by_email,
    structs::{
        Admin, Album, Artist, Collaborator, History, Playlist, Song, SongAlbum, SongPlaylist, User,
        UserLikesAlbum, UserLikesPlaylist, UserLikesSong,
    },
};
use rusqlite::{params, Connection};
use text_io::read;

pub fn select_admin_by_email(conn: &Connection, email: &str) -> Option<Admin> {
    conn.query_row("SELECT id,username,email,password_hash,password_salt,last_connection FROM Admin WHERE email LIKE ?1", params![email], |row| {
        Ok(Admin {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            password_salt: row.get(4)?,
            last_connection: row.get(5)?,
        })
    }).ok()
}

pub fn select_user_by_email(conn: &Connection, email: &str) -> Option<User> {
    conn.query_row("SELECT id,username,email,password_hash,password_salt,last_connection,creation_date,profile_picture FROM User WHERE email LIKE ?1", params![email], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            password_salt: row.get(4)?,
            last_connection: row.get(5)?,
            creation_date: row.get(6)?,
            profile_picture: row.get(7)?,
        })
    }).ok()
}

pub fn select_user_by_id(conn: &Connection, id: i64) -> Option<User> {
    conn.query_row("SELECT id,username,email,password_hash,password_salt,last_connection,creation_date,profile_picture FROM User WHERE id=?1", params![id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            password_salt: row.get(4)?,
            last_connection: row.get(5)?,
            creation_date: row.get(6)?,
            profile_picture: row.get(7)?,
        })
    }).ok()
}

pub fn select_user_by_username(conn: &Connection, username: &str) -> Option<Vec<User>> {
    let mut query = match conn.prepare("SELECT id,username,email,password_hash,password_salt,last_connection,creation_date,profile_picture FROM User WHERE username LIKE ?1") {
        Ok(query) => query,
        Err(_) => return None,
    };

    let user_iter = match query.query_map(params![username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            password_salt: row.get(4)?,
            last_connection: row.get(5)?,
            creation_date: row.get(6)?,
            profile_picture: row.get(7)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for user in user_iter {
        match user {
            Err(_) => return None,
            Ok(user) => res.push(user),
        }
    }

    Some(res)
}

pub fn select_artist_by_email(conn: &Connection, email: &str) -> Option<Artist> {
    if !user_exist_by_email(conn, email) {
        return None;
    }
    conn.query_row(
        "SELECT id,user_id,nb_of_streams,biography,url,verified \
        FROM Artist \
        JOIN User ON Artist.user_id=User.id \
        WHERE User.email LIKE ?1",
        params![email],
        |row| {
            Ok(Artist {
                id: row.get(0)?,
                user_id: row.get(1)?,
                nb_of_streams: row.get(2)?,
                biography: row.get(3)?,
                url: row.get(4)?,
                verified: row.get(5)?,
            })
        },
    )
    .ok()
}

pub fn select_artist_by_id(conn: &Connection, id: i64) -> Option<Artist> {
    conn.query_row(
        "SELECT id,user_id,nb_of_streams,biography,url,verified \
        FROM Artist \
        WHERE Artist.id=?1",
        params![id],
        |row| {
            Ok(Artist {
                id: row.get(0)?,
                user_id: row.get(1)?,
                nb_of_streams: row.get(2)?,
                biography: row.get(3)?,
                url: row.get(4)?,
                verified: row.get(5)?,
            })
        },
    )
    .ok()
}

pub fn select_artist_by_username(conn: &Connection, username: &str) -> Option<Vec<Artist>> {
    let mut query = match conn.prepare(
        "SELECT Artist.id,Artist.user_id,Artist.nb_of_streams,Artist.biography,Artist.url,Artist.verified \
        FROM Artist \
        JOIN User ON Artist.user_id=User.id \
        WHERE User.username=?1") {
        Ok(query) => query,
        Err(_) => return None,
    };

    let artist_iter = match query.query_map(params![username], |row| {
        Ok(Artist {
            id: row.get(0)?,
            user_id: row.get(1)?,
            nb_of_streams: row.get(2)?,
            biography: row.get(3)?,
            url: row.get(4)?,
            verified: row.get(5)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for artist in artist_iter {
        match artist {
            Err(_) => return None,
            Ok(artist) => res.push(artist),
        }
    }

    Some(res)
}

pub fn select_artist_by_user_id(conn: &Connection, user_id: i64) -> Option<Artist> {
    conn.query_row(
        "SELECT Artist.id,Artist.user_id,Artist.nb_of_streams,Artist.biography,Artist.url,Artist.verified \
        FROM Artist \
        WHERE Artist.user_id=?1",
        params![user_id],
        |row| {
            Ok(Artist {
                id: row.get(0)?,
                user_id: row.get(1)?,
                nb_of_streams: row.get(2)?,
                biography: row.get(3)?,
                url: row.get(4)?,
                verified: row.get(5)?,
            })
        },
    )
    .ok()
}

pub fn select_collaborator_by_email(conn: &Connection, email: &str) -> Option<Collaborator> {
    if !user_exist_by_email(conn, email) {
        return None;
    }
    conn.query_row(
        "SELECT id,user_id,verified \
        FROM Collaborator \
        JOIN User ON Collaborator.user_id=User.id \
        WHERE User.email LIKE ?1",
        params![email],
        |row| {
            Ok(Collaborator {
                id: row.get(0)?,
                user_id: row.get(1)?,
                verified: row.get(2)?,
            })
        },
    )
    .ok()
}

pub fn select_collaborator_by_id(conn: &Connection, id: i64) -> Option<Collaborator> {
    conn.query_row(
        "SELECT id,user_id,verified \
        FROM Collaborator \
        WHERE Collaborator.id=?1",
        params![id],
        |row| {
            Ok(Collaborator {
                id: row.get(0)?,
                user_id: row.get(1)?,
                verified: row.get(2)?,
            })
        },
    )
    .ok()
}

pub fn select_song_by_id(conn: &Connection, id: i64) -> Option<Song> {
    conn.query_row(
        "SELECT id,title,song_file,nb_of_streams,cover_image,duration,creation_date,artist_id \
        FROM Song \
        WHERE Song.id=?1",
        params![id],
        |row| {
            Ok(Song {
                id: row.get(0)?,
                title: row.get(1)?,
                song_file: row.get(2)?,
                nb_of_streams: row.get(3)?,
                cover_image: row.get(4)?,
                duration: row.get(5)?,
                creation_date: row.get(6)?,
                artist_id: row.get(7)?,
            })
        },
    )
    .ok()
}

pub fn select_song_by_title(conn: &Connection, title: &str) -> Option<Vec<Song>> {
    let mut query = match conn.prepare(
        "SELECT id,title,song_file,nb_of_streams,cover_image,duration,creation_date,artist_id \
        FROM Song \
        WHERE Song.title LIKE ?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let song_iter = match query.query_map(params![title], |row| {
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            song_file: row.get(2)?,
            nb_of_streams: row.get(3)?,
            cover_image: row.get(4)?,
            duration: row.get(5)?,
            creation_date: row.get(6)?,
            artist_id: row.get(7)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for song in song_iter {
        match song {
            Err(_) => return None,
            Ok(song) => res.push(song),
        }
    }

    Some(res)
}

pub fn select_song_by_artist_id(conn: &Connection, id: i64) -> Option<Vec<Song>> {
    let mut query = match conn.prepare(
        "SELECT id,title,song_file,nb_of_streams,cover_image,duration,creation_date,artist_id \
        FROM Song \
        WHERE Song.artist_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let song_iter = match query.query_map(params![id], |row| {
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            song_file: row.get(2)?,
            nb_of_streams: row.get(3)?,
            cover_image: row.get(4)?,
            duration: row.get(5)?,
            creation_date: row.get(6)?,
            artist_id: row.get(7)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for song in song_iter {
        match song {
            Err(_) => return None,
            Ok(song) => res.push(song),
        }
    }

    Some(res)
}

pub fn select_album_by_id(conn: &Connection, id: i64) -> Option<Album> {
    conn.query_row(
        "SELECT id,title,cover_image,creation_date,artist_id \
        FROM Album \
        WHERE Album.id=?1",
        params![id],
        |row| {
            Ok(Album {
                id: row.get(0)?,
                title: row.get(1)?,
                cover_image: row.get(2)?,
                creation_date: row.get(3)?,
                artist_id: row.get(4)?,
            })
        },
    )
    .ok()
}

pub fn select_album_by_title(conn: &Connection, title: &str) -> Option<Vec<Album>> {
    let mut query = match conn.prepare(
        "SELECT id,title,cover_image,creation_date,artist_id \
        FROM Album \
        WHERE Album.title LIKE ?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let album_iter = match query.query_map(params![title], |row| {
        Ok(Album {
            id: row.get(0)?,
            title: row.get(1)?,
            cover_image: row.get(2)?,
            creation_date: row.get(3)?,
            artist_id: row.get(4)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for album in album_iter {
        match album {
            Err(_) => return None,
            Ok(album) => res.push(album),
        }
    }

    Some(res)
}

pub fn select_album_by_artist_id(conn: &Connection, id: i64) -> Option<Vec<Album>> {
    let mut query = match conn.prepare(
        "SELECT id,title,cover_image,creation_date,artist_id \
        FROM Album \
        WHERE Album.artist_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let album_iter = match query.query_map(params![id], |row| {
        Ok(Album {
            id: row.get(0)?,
            title: row.get(1)?,
            cover_image: row.get(2)?,
            creation_date: row.get(3)?,
            artist_id: row.get(4)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for album in album_iter {
        match album {
            Err(_) => return None,
            Ok(album) => res.push(album),
        }
    }

    Some(res)
}

pub fn select_playlist_by_id(conn: &Connection, id: i64) -> Option<Playlist> {
    conn.query_row(
        "SELECT id,title,cover_image,creation_date,user_id \
        FROM Playlist \
        WHERE Playlist.id=?1",
        params![id],
        |row| {
            Ok(Playlist {
                id: row.get(0)?,
                title: row.get(1)?,
                cover_image: row.get(2)?,
                creation_date: row.get(3)?,
                user_id: row.get(4)?,
            })
        },
    )
    .ok()
}

pub fn select_playlist_by_title(conn: &Connection, title: &str) -> Option<Vec<Playlist>> {
    let mut query = match conn.prepare(
        "SELECT id,title,cover_image,creation_date,user_id \
        FROM Playlist \
        WHERE Playlist.title LIKE ?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let playlist_iter = match query.query_map(params![title], |row| {
        Ok(Playlist {
            id: row.get(0)?,
            title: row.get(1)?,
            cover_image: row.get(2)?,
            creation_date: row.get(3)?,
            user_id: row.get(4)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for playlist in playlist_iter {
        match playlist {
            Err(_) => return None,
            Ok(playlist) => res.push(playlist),
        }
    }

    Some(res)
}

pub fn select_playlist_by_user_id(conn: &Connection, id: i64) -> Option<Vec<Playlist>> {
    let mut query = match conn.prepare(
        "SELECT id,title,cover_image,creation_date,user_id \
        FROM Playlist \
        WHERE Playlist.user_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let playlist_iter = match query.query_map(params![id], |row| {
        Ok(Playlist {
            id: row.get(0)?,
            title: row.get(1)?,
            cover_image: row.get(2)?,
            creation_date: row.get(3)?,
            user_id: row.get(4)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for playlist in playlist_iter {
        match playlist {
            Err(_) => return None,
            Ok(playlist) => res.push(playlist),
        }
    }

    Some(res)
}

pub fn select_user_likes_song_by_user_id(conn: &Connection, id: i64) -> Option<Vec<UserLikesSong>> {
    let mut query = match conn.prepare(
        "SELECT user_id,song_id \
        FROM UserLikesSong \
        WHERE UserLikesSong.user_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(UserLikesSong {
            user_id: row.get(0)?,
            song_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}
pub fn select_user_likes_song_by_song_id(conn: &Connection, id: i64) -> Option<Vec<UserLikesSong>> {
    let mut query = match conn.prepare(
        "SELECT user_id,song_id \
        FROM UserLikesSong \
        WHERE UserLikesSong.song_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(UserLikesSong {
            user_id: row.get(0)?,
            song_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}

pub fn select_user_likes_album_by_user_id(
    conn: &Connection,
    id: i64,
) -> Option<Vec<UserLikesAlbum>> {
    let mut query = match conn.prepare(
        "SELECT user_id,album_id \
        FROM UserLikesAlbum \
        WHERE UserLikesAlbum.user_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(UserLikesAlbum {
            user_id: row.get(0)?,
            album_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}
pub fn select_user_likes_album_by_album_id(
    conn: &Connection,
    id: i64,
) -> Option<Vec<UserLikesAlbum>> {
    let mut query = match conn.prepare(
        "SELECT user_id,album_id \
        FROM UserLikesAlbum \
        WHERE UserLikesAlbum.album_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(UserLikesAlbum {
            user_id: row.get(0)?,
            album_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}

pub fn select_user_likes_playlist_by_user_id(
    conn: &Connection,
    id: i64,
) -> Option<Vec<UserLikesPlaylist>> {
    let mut query = match conn.prepare(
        "SELECT user_id,playlist_id \
        FROM UserLikesPlaylist \
        WHERE UserLikesPlaylist.user_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(UserLikesPlaylist {
            user_id: row.get(0)?,
            playlist_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}
pub fn select_user_likes_playlist_by_playlist_id(
    conn: &Connection,
    id: i64,
) -> Option<Vec<UserLikesPlaylist>> {
    let mut query = match conn.prepare(
        "SELECT user_id,playlist_id \
        FROM UserLikesPlaylist \
        WHERE UserLikesPlaylist.playlist_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(UserLikesPlaylist {
            user_id: row.get(0)?,
            playlist_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}

pub fn select_song_album_by_song_id(conn: &Connection, id: i64) -> Option<Vec<SongAlbum>> {
    let mut query = match conn.prepare(
        "SELECT song_id,album_id \
        FROM SongAlbum \
        WHERE SongAlbum.song_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(SongAlbum {
            song_id: row.get(0)?,
            album_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}
pub fn select_song_album_by_album_id(conn: &Connection, id: i64) -> Option<Vec<SongAlbum>> {
    let mut query = match conn.prepare(
        "SELECT song_id,album_id \
        FROM SongAlbum \
        WHERE SongAlbum.album_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(SongAlbum {
            song_id: row.get(0)?,
            album_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}

pub fn select_song_playlist_by_song_id(conn: &Connection, id: i64) -> Option<Vec<SongPlaylist>> {
    let mut query = match conn.prepare(
        "SELECT song_id,album_id \
        FROM SongPlaylist \
        WHERE SongPlaylist.song_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(SongPlaylist {
            song_id: row.get(0)?,
            playlist_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}
pub fn select_song_playlist_by_playlist_id(
    conn: &Connection,
    id: i64,
) -> Option<Vec<SongPlaylist>> {
    let mut query = match conn.prepare(
        "SELECT song_id,album_id \
        FROM SongPlaylist \
        WHERE SongPlaylist.playlist_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(SongPlaylist {
            song_id: row.get(0)?,
            playlist_id: row.get(1)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}

pub fn select_history_by_user_id(conn: &Connection, id: i64) -> Option<Vec<History>> {
    let mut query = match conn.prepare(
        "SELECT user_id,song_id,time \
        FROM History \
        WHERE History.user_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(History {
            user_id: row.get(0)?,
            song_id: row.get(1)?,
            time: row.get(2)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}
pub fn select_history_by_song_id(conn: &Connection, id: i64) -> Option<Vec<History>> {
    let mut query = match conn.prepare(
        "SELECT user_id,song_id,time \
        FROM History \
        WHERE History.song_id=?1",
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![id], |row| {
        Ok(History {
            user_id: row.get(0)?,
            song_id: row.get(1)?,
            time: row.get(2)?,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut res = Vec::new();
    for elt in iter {
        match elt {
            Err(_) => return None,
            Ok(elt) => res.push(elt),
        }
    }

    Some(res)
}

// =================================================================== DEV ZONE

pub fn select_usernames(conn: Connection) -> Option<Vec<(i64, String)>> {
    let mut format = match conn.prepare("SELECT id, username, email, password_hash, password_salt, last_connection, creation_date, profile_picture FROM User") {
        Ok(fmt) => fmt,
        Err(_) => return None,
    };
    let user_iter = match format.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: String::new(),
            password_hash: String::new(),
            password_salt: String::new(),
            last_connection: 0,
            creation_date: 0,
            profile_picture: None,
        })
    }) {
        Ok(it) => it,
        Err(_) => return None,
    };

    let mut usernames = Vec::new();
    for user in user_iter {
        match user {
            Ok(user) => usernames.push((user.id, user.username)),
            Err(_) => return None,
        }
    }

    Some(usernames)
}
pub fn dev_select_user_by_email(conn: Connection) -> Option<User> {
    print!("Enter searched email: ");
    let input: String = read!();
    select_user_by_email(&conn, &input)
}
