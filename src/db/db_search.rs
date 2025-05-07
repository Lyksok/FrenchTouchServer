use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongSearch {
    pub song_id: i64,
    pub song_title: String,
    pub song_cover: Option<String>,
    pub artist_name: String,
}

pub fn select_search_song(conn: &Connection, title: &str) -> Option<Vec<SongSearch>> {
    let mut query = match conn.prepare(
        "SELECT Song.id,Song.title,Song.cover_image,User.username \
        FROM Song \
        JOIN Artist ON Artist.id=Song.artist_id \
        JOIN User ON Artist.user_id=User.id \
        WHERE LOWER(Song.title) LIKE LOWER(?)"
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let formated_title = format!("%{}%", title);
    let iter = match query.query_map(params![formated_title], |row| {
        Ok(SongSearch{
            song_id: row.get(0)?,
            song_title: row.get(1)?,
            song_cover: row.get(2)?,
            artist_name: row.get(3)?,
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

pub fn select_search_song_by_artist_id(conn: &Connection, artist_id: i64) -> Option<Vec<SongSearch>> {
    let mut query = match conn.prepare(
        "SELECT Song.id,Song.title,Song.cover_image,User.username \
        FROM Song \
        JOIN Artist ON Artist.id=Song.artist_id \
        JOIN User ON Artist.user_id=User.id \
        WHERE Song.artist_id=?"
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let iter = match query.query_map(params![artist_id], |row| {
        Ok(SongSearch{
            song_id: row.get(0)?,
            song_title: row.get(1)?,
            song_cover: row.get(2)?,
            artist_name: row.get(3)?,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistSearch {
    pub artist_id: i64,
    pub artist_name: String,
    pub artist_cover: Option<String>,
}

pub fn select_search_artist(conn: &Connection, name: &str) -> Option<Vec<ArtistSearch>> {
    let mut query = match conn.prepare(
        "SELECT Artist.id,User.username,User.profile_picture \
        FROM Artist \
        JOIN User ON Artist.user_id=User.id \
        WHERE LOWER(User.username) LIKE LOWER(?)"
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let formated_title = format!("%{}%", name);
    let iter = match query.query_map(params![formated_title], |row| {
        Ok(ArtistSearch{
            artist_id: row.get(0)?,
            artist_name: row.get(1)?,
            artist_cover: row.get(2)?,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumSearch {
    pub album_id: i64,
    pub album_name: String,
    pub album_cover: Option<String>,
    pub artist_name: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistSearch {
    pub playlist_id: i64,
    pub playlist_name: String,
    pub playlist_cover: Option<String>,
    pub user_name: i64,
}

pub fn select_search_album(conn: &Connection, name: &str) -> Option<Vec<AlbumSearch>> {
    let mut query = match conn.prepare(
        "SELECT Album.id,Album.title,Album.cover_image,User.username \
        FROM Album \
        JOIN Artist ON Album.artist_id=Artist.id \
        JOIN User ON Artist.user_id=User.id \
        WHERE LOWER(Album.title) LIKE LOWER(?)"
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let formated_title = format!("%{}%", name);
    let iter = match query.query_map(params![formated_title], |row| {
        Ok(AlbumSearch{
            album_id: row.get(0)?,
            album_name: row.get(1)?,
            album_cover: row.get(2)?,
            artist_name: row.get(3)?,
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

pub fn select_search_playlist(conn: &Connection, name: &str) -> Option<Vec<PlaylistSearch>> {
    let mut query = match conn.prepare(
        "SELECT Playlist.id,Playlist.title,Playlist.cover_image,User.username \
        FROM Playlist \
        JOIN User ON Playlist.user_id=User.id \
        WHERE LOWER(Playlist.title) LIKE LOWER(?)"
    ) {
        Ok(query) => query,
        Err(_) => return None,
    };

    let formated_title = format!("%{}%", name);
    let iter = match query.query_map(params![formated_title], |row| {
        Ok(PlaylistSearch{
            playlist_id: row.get(0)?,
            playlist_name: row.get(1)?,
            playlist_cover: row.get(2)?,
            user_name: row.get(3)?,
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