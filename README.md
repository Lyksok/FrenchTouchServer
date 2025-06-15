# French Touch Server

French Touch Server is a server written in Rust. Its purpose is to give French Touch clients access to a centralized database and file storage. It does not compute nor handle anything except single accesses to the database (insert, files, select, update, delete).

# Installation/Setup

## 1. Clone the project and cd into it
```
git clone https://github.com/Lyksok/FrenchTouchServer.git
cd FrenchTouchServer
```
## 2. Build the server in release mode
```
cargo build --release
```
## 3. Run the server
Before running the server, you will probably need to generate a self-signed certificate
First change the `openssl.cnf` file to match your needs and then run the following.
```
mkdir ssl
./generate-certificates.sh
```
You can then run the server with the following command.
```
cargo run --release
```
Alternatively, using `screen` package, you can run the server by running
```
./start.sh
```
And then
```
screen -r french-touch
```
To exit, you can simply close the server (Ctrl+c then choose the right option) or if you want it to run in background and switch to your shell, use **Ctrl+a d**.

## 4. Open port
To be able to access your server from anywhere, you need to open your ports.
You need to open your server port (defined by default to `50000` in src/api/run_api.rs) to public internet through your internet provider's box. Check online tutorials.

# API access

## INSERT

- [X] /insert/admin _(AdminRequest)_
- [X] /insert/user _(User)_
- [X] /insert/artist _(ArtistObjRequest)_
- [X] /insert/collaborator _(CollaboratorObjRequest)_
- [X] /insert/song _(SongRequest)_
- [X] /insert/album _(AlbumRequest)_
- [X] /insert/playlist _(PlaylistRequest)_
- [X] /insert/user-likes-song _(UserLikesSongRequest)_
- [X] /insert/user-likes-album _(UserLikesAlbumRequest)_
- [X] /insert/user-likes-playlist _(UserLikesPlaylistRequest)_
- [X] /insert/song-album _(SongAlbumRequest)_
- [X] /insert/song-playlist _(SongPlaylistRequest)_
- [X] /insert/history _(HistoryRequest)_
- [X] /insert/collaborator_request _(CollaboratorRequestRequest)_
- [X] /insert/artist_request _(ArtistRequestRequest)_
- [X] /insert/request_to_artist _(WrapperRequestToArtist)_
- [X] /insert/request_to_collaborator _(WrapperRequestToCollaborator)_
- [X] /insert/request_to_admin _(WrapperRequestToAdmin)_


## FILES

- [X] /post/image
- [X] /get/image/{filename}
- [X] /post/song
- [X] /get/song/{filename}

## SELECT

### Admin
- [X] /select/admin/user_id/{user_id} + _AuthHash_

### User
- [X] /select/user/email/{email}
- [X] /select/user/id/{id}
- [X] /select/user/username/{username}

### Artist 
- [X] /select/artist/all
- [X] /select/artist/email/{email}
- [X] /select/artist/id/{id}
- [X] /select/artist/username/{username}
- [X] /select/artist/user_id/{user_id}

### Collaborator
- [X] /select/collaborator/email/{email}
- [X] /select/collaborator/id/{id}
- [X] /select/collaborator/user_id/{user_id}

### Song
- [X] /select/song/id/{id}
- [X] /select/song/title/{title}
- [X] /select/song/artist_id/{artist_id}

### Album
- [X] /select/album/id/{id}
- [X] /select/album/title/{title}
- [X] /select/album/artist_id/{artist_id}

### Playlist
- [X] /select/playlist/id/{id}
- [X] /select/playlist/title/{title}
- [X] /select/playlist/user_id/{user_id}

### UserLikesSong
- [X] /select/user_likes_song/user_id/{user_id}
- [X] /select/user_likes_song/song_id/{song_id}

### UserLikesAlbum
- [X] /select/user_likes_album/user_id/{user_id}
- [X] /select/user_likes_album/album_id/{album_id}

### UserLikesPlaylist
- [X] /select/user_likes_playlist/user_id/{user_id}
- [X] /select/user_likes_playlist/playlist_id/{playlist_id}

### SongAlbum
- [X] /select/song_album/song_id/{song_id}
- [X] /select/song_album/album_id/{album_id}

### SongPlaylist
- [X] /select/song_playlist/song_id/{song_id}
- [X] /select/song_playlist/playlist_id/{playlist_id}

### History
- [X] /select/history/user_id/{user_id}
- [X] /select/history/song_id/{song_id}

### ArtistRequest
- [X] /select/artist_request/id/{id}
- [X] /select/artist_request/all

### CollaboratorRequest
- [X] /select/collaborator_request/id/{id}
- [X] /select/collaborator_request/all

### RequestToArtist
- [X] /select/request_to_artist/user_id/{user_id}
- [X] /select/request_to_artist/all

### RequestToCollaborator
- [X] /select/request_to_collaborator/user_id/{user_id}
- [X] /select/request_to_collaborator/all

### RequestToArtist
- [X] /select/request_to_admin/user_id/{user_id}
- [X] /select/request_to_admin/all

## UPDATE

### User
- [X] /update/user/profile-picture/{email} _(UserRequest)_
- [X] /update/user/last-connection/{email} _(UserRequest)_
- [X] /update/user/password/{id} _(AuthInfoNewPassword)_

### CollaboratorRequest
- [X] /update/collaborator_request/id _(CollaboratorRequestRequest)_

## DELETE

### CollaboratorRequest
- [X] /delete/collaborator_request/id/{id} + _AuthHash_

### ArtistRequest
- [X] /delete/artist_request/id/{id} + _AuthHash_

### RequestToCollaborator
- [X] /delete/request_to_collaborator/user_id/{user_id} + _AuthHash_

### RequestToArtist
- [X] /delete/request_to_artist/user_id/{user_id} + _AuthHash_

### RequestToAdmin
- [X] /delete/request_to_admin/user_id/{user_id} + _AuthHash_
