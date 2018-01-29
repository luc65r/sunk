use serde_json;

use error::{ApiError, Error, Result};

/// A top-level response from a Subsonic server.
#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "subsonic-response")]
    pub inner: InnerResponse,
}

/// A struct containing the possible responses of the Subsonic API.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerResponse {
    status: String,
    version: String,
    #[doc(hidden)]
    error: Option<ApiError>,
    license: Option<serde_json::Value>,
    music_folders: Option<serde_json::Value>,
    indexes: Option<serde_json::Value>,
    directory: Option<serde_json::Value>,
    genres: Option<serde_json::Value>,
    artists: Option<serde_json::Value>,
    artist: Option<serde_json::Value>,
    albums: Option<serde_json::Value>,
    album: Option<serde_json::Value>,
    song: Option<serde_json::Value>,
    videos: Option<serde_json::Value>,
    video_info: Option<serde_json::Value>,
    artist_info: Option<serde_json::Value>,
    artist_info2: Option<serde_json::Value>,
    album_info: Option<serde_json::Value>,
    similar_songs: Option<serde_json::Value>,
    similar_songs2: Option<serde_json::Value>,
    top_songs: Option<serde_json::Value>,
    album_list: Option<serde_json::Value>,
    album_list2: Option<serde_json::Value>,
    random_songs: Option<serde_json::Value>,
    songs_by_genre: Option<serde_json::Value>,
    now_playing: Option<serde_json::Value>,
    starred: Option<serde_json::Value>,
    starred2: Option<serde_json::Value>,
    search_result: Option<serde_json::Value>,
    search_result2: Option<serde_json::Value>,
    search_result3: Option<serde_json::Value>,
    playlists: Option<serde_json::Value>,
    playlist: Option<serde_json::Value>,
    lyrics: Option<serde_json::Value>,
    shares: Option<serde_json::Value>,
    podcasts: Option<serde_json::Value>,
    newest_podcasts: Option<serde_json::Value>,
    jukebox_status: Option<serde_json::Value>,
    jukebox_playlist: Option<serde_json::Value>,
    internet_radio_stations: Option<serde_json::Value>,
    chat_messages: Option<serde_json::Value>,
    user: Option<serde_json::Value>,
    users: Option<serde_json::Value>,
    bookmarks: Option<serde_json::Value>,
    play_queue: Option<serde_json::Value>,
    scan_status: Option<serde_json::Value>,
}

impl Response {
    /// Extracts the internal value of the response.
    ///
    /// # Errors
    ///
    /// This method will error if the response contained an error (as defined by
    /// the [Subsonic API]).
    ///
    /// [Subsonic API]: ../error/enum.ApiError.html
    pub fn into_value(self) -> Result<serde_json::Value> {
        // TODO Big time; make this not an `if ... else if ...` mess.
        macro_rules! maybe {
            ($f:ident) => ({
                if let Some(v)  = self.inner.$f {
                    return Ok(v)
                }
            })
        }

        if let Some(err) = self.inner.error {
            return Err(err.into())
        }

        maybe!(license);
        maybe!(music_folders);
        maybe!(music_folders);
        maybe!(indexes);
        maybe!(directory);
        maybe!(genres);
        maybe!(artists);
        maybe!(artist);
        maybe!(albums);
        maybe!(album);
        maybe!(song);
        maybe!(videos);
        maybe!(video_info);
        maybe!(artist_info);
        maybe!(artist_info2);
        maybe!(album_info);
        maybe!(similar_songs);
        maybe!(similar_songs2);
        maybe!(top_songs);
        maybe!(album_list);
        maybe!(album_list2);
        maybe!(random_songs);
        maybe!(songs_by_genre);
        maybe!(now_playing);
        maybe!(starred);
        maybe!(starred2);
        maybe!(search_result);
        maybe!(search_result2);
        maybe!(search_result3);
        maybe!(playlists);
        maybe!(playlist);
        maybe!(lyrics);
        maybe!(shares);
        maybe!(podcasts);
        maybe!(newest_podcasts);
        maybe!(jukebox_status);
        maybe!(jukebox_playlist);
        maybe!(internet_radio_stations);
        maybe!(chat_messages);
        maybe!(user);
        maybe!(users);
        maybe!(bookmarks);
        maybe!(play_queue);
        maybe!(scan_status);

        Err(Error::Other("non-exhaustive `into_value()`"))
    }

    /// Extracts the error struct of the response. Returns `None` if the
    /// response was not a failure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// #[macro_use]
    /// extern crate serde_json;
    /// extern crate sunk;
    /// use sunk::response::Response;
    ///
    /// # fn run() -> Result<(), sunk::Error> {
    /// let fail = json!({"subsonic-response": {
    ///     "status": "failed",
    ///     "version": "1.14.0",
    ///     "error": {
    ///         "code": 70,
    ///         "message": "Requested resource not found"
    ///     }
    /// }});
    /// let fail = serde_json::from_value::<Response>(fail)?;
    /// assert!(fail.into_error().is_some());
    ///
    /// let success = json!({"subsonic-response": {
    ///     "status": "ok",
    ///     "version": "1.14.0"
    /// }});
    /// let success = serde_json::from_value::<Response>(success)?;
    /// assert!(success.into_error().is_none());
    /// # Ok(())
    /// # }
    /// # fn main() {
    /// #   run().unwrap();
    /// # }
    /// ```
    pub fn into_error(self) -> Option<Error> {
        self.inner.error.map(|e| e.into())
    }

    /// Returns `true` if the response is `"ok"`.
    pub fn is_ok(&self) -> bool { self.inner.status == "ok" }

    /// Returns `true` if the response is `"failed"`.
    pub fn is_err(&self) -> bool { self.inner.status == "failed" }
}
