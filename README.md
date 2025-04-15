# French Touch Server

French Touch Server is a server written in Rust. Its purpose is to give French Touch clients access to a centralized database and file storage. It does not compute nor handle anything except single accesses to the database (insert, files, select, update, delete).

# API access

## INSERT

-[X] User               (/user/insert)
-[X] Artist             (/artist/insert)
-[X] Collaborator       (/collaborator/insert)
-[X] Song               (/song/insert)
-[X] Album              (/album/insert)
-[X] Playlist           (/playlist/insert)
-[X] UserLikesSong      (/user-likes-song/insert)
-[ ] UserLikesAlbum     (/user-likes-album/insert)
-[ ] UserLikesPlaylist  (/user-likes-playlist/insert)
-[ ] SongAlbum          (/song-album/insert)
-[ ] SongPlaylist       (/song-playlist/insert)
-[ ] History            (/history/insert)

## FILES

-[X] SaveImage  (/image/post)
-[X] GetImage   (/image/get/{filename})
-[X] SaveSong   (/song/post)
-[X] GetSong    (/song/get/{filename})

## SELECT

### Admin
-[ ] EMAIL      (/admin/select/email/{email})

### User
-[X] EMAIL      (/user/select/email/{email})
-[ ] ID         (/user/select/id/{id})
-[ ] USERNAME   (/user/select/username/{username})

### Artist 
-[ ] EMAIL      (/artist/select/email/{email})
-[ ] ID         (/artist/select/id/{id})
-[ ] USERNAME   (/artist/select/username/{username})

### Collaborator
-[ ] EMAIL      (/collaborator/select/email/{email})
-[ ] ID         (/collaborator/select/id/{id})

### Song
-[ ] ID         (/song/select/id/{id})
-[ ] TITLE      (/song/select/title/{title})
-[ ] ARTISTID   (/song/select/artist-id/{artist-id})

### Album
-[ ] ID         (/album/select/id/{id})
-[ ] TITLE      (/album/select/title/{title})
-[ ] ARTISTID   (/album/select/artist-id/{artist-id})

### Playlist
-[ ] ID         (/playlist/select/id/{id})
-[ ] TITLE      (/playlist/select/title/{title})
-[ ] USERID     (/playlist/select/user-id/{user-id})

### UserLikesSong
-[ ] USERID     (/user-likes-song/select/user-id/{user-id})
-[ ] SONGID     (/user-likes-song/select/song-id/{song-id})

### UserLikesAlbum
-[ ] USERID     (/user-likes-album/select/user-id/{user-id})
-[ ] ALBUMID    (/user-likes-album/select/album-id/{album-id})

### UserLikesPlaylist
-[ ] USERID     (/user-likes-playlist/select/user-id/{user-id})
-[ ] PLAYLISTID (/user-likes-playlist/select/playlist-id/{playlist-id})

### SongAlbum
-[ ] ALBUMID    (/song-album/select/album-id/{album-id})

### SongPlaylist
-[ ] SONGID     (/song-playlist/select/song-id/{song-id})
-[ ] PLAYLISTID (/song-playlist/select/playlist-id/{playlist-id})

### History
-[ ] USERID     (/history/select/user-id/{user-id})
-[ ] SONGID     (/history/select/song-id/{song-id})

## UPDATE

### User
### Artist
### Collaborator
### Song
### Album
### Playlist

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

# DB access

### Admin
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
