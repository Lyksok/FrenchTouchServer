# French Touch Server

French Touch Server is a server written in Rust. Its purpose is to give French Touch clients access to a centralized database and file storage. It does not compute nor handle anything except single accesses to the database (insert, files, select, update, delete).

# API access

## INSERT

- [X] /insert/user
- [X] /insert/artist
- [X] /insert/collaborator
- [X] /insert/song
- [X] /insert/album
- [X] /insert/playlist
- [X] /insert/user-likes-song
- [X] /insert/user-likes-album
- [X] /insert/user-likes-playlist
- [X] /insert/song-album
- [X] /insert/song-playlist
- [X] /insert/history

## FILES

- [X] /get/image/{filename}
- [X] /get/song/{filename}
- [X] /post/image
- [X] /post/song

## SELECT

### Admin
- [X] /select/admin/email/{email}

### User
- [X] /select/user/email/{email}
- [ ] /select/user/id/{id}
- [ ] /select/user/username/{username}

### Artist 
- [ ] /select/artist/email/{email}
- [ ] /select/artist/id/{id}
- [ ] /select/artist/username/{username}

### Collaborator
- [ ] /select/collaborator/email/{email}
- [ ] /select/collaborator/id/{id}

### Song
- [ ] /select/song/id/{id}
- [ ] /select/song/title/{title}
- [ ] /select/song/artist-id/{artist-id}

### Album
- [ ] /select/album/id/{id}
- [ ] /select/album/title/{title}
- [ ] /select/album/artist-id/{artist-id}

### Playlist
- [ ] /select/playlist/id/{id}
- [ ] /select/playlist/title/{title}
- [ ] /select/playlist/user-id/{user-id}

### UserLikesSong
- [ ] /select/user-likes-song/user-id/{user-id}
- [ ] /select/user-likes-song/song-id/{song-id}

### UserLikesAlbum
- [ ] /select/user-likes-album/user-id/{user-id}
- [ ] /select/user-likes-album/album-id/{album-id}

### UserLikesPlaylist
- [ ] /select/user-likes-playlist/user-id/{user-id}
- [ ] /select/user-likes-playlist/playlist-id/{playlist-id}

### SongAlbum
- [ ] /select/song-album/album-id/{album-id}

### SongPlaylist
- [ ] /select/song-playlist/song-id/{song-id}
- [ ] /select/song-playlist/playlist-id/{playlist-id}

### History
- [ ] /select/history/user-id/{user-id}
- [ ] /select/history/song-id/{song-id}

## UPDATE

### User
- [ ] /update/user/username/{email}
- [ ] /update/user/email/{email}
- [ ] /update/user/credentials/{email}
- [ ] /update/user/last-connection/{email}
- [X] /update/user/profile-picture/{email}

### Artist
- [ ] /update/artist/nb-of-streams/{id}
- [ ] /update/artist/biography/{id}
- [ ] /update/artist/url/{id}
- [ ] /update/artist/verified/{id}

### Collaborator
- [ ] /update/collaborator/verified/{id}

### Song
- [ ] /update/song/title/{id}
- [ ] /update/song/song-file/{id}
- [ ] /update/song/nb-of-streams/{id}
- [ ] /update/song/cover-image/{id}
- [ ] /update/song/duration/{id}
- [ ] /update/song/creation-date/{id}

### Album
- [ ] /update/album/title/{id}
- [ ] /update/album/cover-image/{id}
- [ ] /update/album/creation-date/{id}

### Playlist
- [ ] /update/playlist/title/{id}
- [ ] /update/playlist/cover-image/{id}
- [ ] /update/playlist/creation-date/{id}

## DELETE

### User
### Artist
### Collaborator
### Song
### Album
### Playlist
### UserLikesSong
### UserLikesAlbum
### UserLikesPlaylist
### SongAlbum
### SongPlaylist
### History
