CREATE TABLE Admin (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    last_connection INTEGER
);

CREATE TABLE User (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    last_connection INTEGER,
    creation_date INTEGER,
    profile_picture TEXT
);

CREATE TABLE Artist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    nb_of_streams INTEGER DEFAULT 0,
    biographie TEXT,
    url TEXT,
    verified INTEGER DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES User(id)
);

CREATE TABLE Collaborator (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    verified INTEGER DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES User(id)
);

CREATE TABLE Song (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    song_file TEXT NOT NULL,
    nb_of_streams INTEGER DEFAULT 0,
    cover_image TEXT,
    duration INTEGER,
    creation_date INTEGER,
    artist_id INTEGER,
    FOREIGN KEY (artist_id) REFERENCES Artist(id)
);

CREATE TABLE Album (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    cover TEXT,
    creation_date INTEGER,
    artist_id INTEGER,
    FOREIGN KEY (artist_id) REFERENCES Artist(id)
);

CREATE TABLE Playlist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    cover TEXT,
    creation_date INTEGER,
    user_id INTEGER,
    FOREIGN KEY (user_id) REFERENCES User(id)
);

CREATE TABLE UserLikesSong (
    user_id INTEGER,
    song_id INTEGER,
    PRIMARY KEY (user_id, song_id),
    FOREIGN KEY (user_id) REFERENCES User(id),
    FOREIGN KEY (song_id) REFERENCES Song(id)
);

CREATE TABLE UserLikesAlbum (
    user_id INTEGER,
    album_id INTEGER,
    PRIMARY KEY (user_id, album_id),
    FOREIGN KEY (user_id) REFERENCES User(id),
    FOREIGN KEY (album_id) REFERENCES Album(id)
);

CREATE TABLE UserLikesPlaylist (
    user_id INTEGER,
    playlist_id INTEGER,
    PRIMARY KEY (user_id, playlist_id),
    FOREIGN KEY (user_id) REFERENCES User(id),
    FOREIGN KEY (playlist_id) REFERENCES Playlist(id)
);

CREATE TABLE SongAlbum (
    song_id INTEGER,
    album_id INTEGER,
    PRIMARY KEY (song_id, album_id),
    FOREIGN KEY (song_id) REFERENCES Song(id),
    FOREIGN KEY (album_id) REFERENCES Album(id)
);

CREATE TABLE SongPlaylist (
    song_id INTEGER,
    playlist_id INTEGER,
    PRIMARY KEY (song_id, playlist_id),
    FOREIGN KEY (song_id) REFERENCES Song(id),
    FOREIGN KEY (playlist_id) REFERENCES Playlist(id)
);

CREATE TABLE History (
    user_id INTEGER
    song_id INTEGER,
    time INTEGER,
    PRIMARY KEY (user_id, song_id, time),
    FOREIGN KEY (user_id) REFERENCES User(id),
    FOREIGN KEY (song_id) REFERENCES Song(id),
);
