use std::result;

use serde::de::{Deserialize, Deserializer};
use serde_json;

use {Album, Client, Error, Media, Result, Song};
use query::Query;

/// Basic information about an artist.
#[derive(Debug)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    cover_id: Option<String>,
    albums: Vec<Album>,
    pub album_count: u64,
}

/// Detailed information about an artist.
#[derive(Debug)]
pub struct ArtistInfo {
    /// A blurb about the artist.
    pub biography: String,
    /// The artist's [MusicBrainz](https://musicbrainz.org/) ID.
    pub musicbrainz_id: String,
    /// The artist's [last.fm](https://last.fm) landing page.
    pub lastfm_url: String,
    /// URLs for the artist's image; available in small, medium, and large.
    pub image_urls: (String, String, String),
    /// Artists similar to this one. Provided by last.fm.
    pub similar_artists: Vec<SimilarArtist>,
}

/// An artist suggested by last.fm.
#[derive(Debug)]
pub struct SimilarArtist {
    id: u64,
    /// The artist's name.
    pub name: String,
    cover_art: Option<String>,
    /// The number of albums contained in the Subsonic server released by the artist.
    pub album_count: u64,
}

impl<'de> Deserialize<'de> for SimilarArtist {
    fn deserialize<D>(de: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _SimilarArtist {
            id: String,
            name: String,
            cover_art: Option<String>,
            album_count: String,
        }

        let raw = _SimilarArtist::deserialize(de)?;

        Ok(SimilarArtist {
            id: raw.id.parse().unwrap(),
            name: raw.name,
            cover_art: raw.cover_art,
            album_count: raw.album_count.parse().unwrap(),
        })
    }
}

impl Artist {
    /// Returns a list of albums released by the artist.
    pub fn albums(&self, client: &Client) -> Result<Vec<Album>> {
        if self.albums.len() as u64 != self.album_count {
            Ok(get_artist(client, self.id)?.albums)
        } else {
            Ok(self.albums.clone())
        }
    }

    /// Queries last.fm for more information about the artist.
    ///
    /// Optionally accepts a maximum number of similar artists to return, and whether to include
    /// artists that are not present on the Subsonic server.
    pub fn info<B, U>(
        &self,
        client: &Client,
        count: U,
        include_not_present: B,
    ) -> Result<ArtistInfo>
    where
        B: Into<Option<bool>>,
        U: Into<Option<usize>>,
    {
        let args = Query::with("id", self.id)
            .arg("count", count.into())
            .arg("includeNotPresent", include_not_present.into())
            .build();
        let res = client.get("getArtistInfo", args)?;
        Ok(serde_json::from_value(res)?)
    }

    /// Returns the top `count` most played songs released by the artist.
    pub fn top_songs<U>(&self, client: &Client, count: U) -> Result<Vec<Song>>
    where
        U: Into<Option<usize>>,
    {
        let args = Query::with("id", self.id)
            .arg("count", count.into())
            .build();

        let song = client.get("getTopSongs", args)?;
        Ok(get_list_as!(song, Song))
    }
}

impl<'de> Deserialize<'de> for Artist {
    fn deserialize<D>(de: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _Artist {
            id: String,
            name: String,
            cover_art: Option<String>,
            album_count: u64,
            #[serde(default)]
            album: Vec<Album>,
        }

        let raw = _Artist::deserialize(de)?;

        Ok(Artist {
            id: raw.id.parse().unwrap(),
            name: raw.name,
            cover_id: raw.cover_art,
            album_count: raw.album_count,
            albums: raw.album,
        })
    }
}

impl Media for Artist {
    fn has_cover_art(&self) -> bool { self.cover_id.is_some() }

    fn cover_id(&self) -> Option<&str> {
        self.cover_id.as_ref().map(|s| s.as_str())
    }

    fn cover_art<U: Into<Option<usize>>>(
        &self,
        client: &Client,
        size: U,
    ) -> Result<Vec<u8>> {
        let cover = self.cover_id()
            .ok_or_else(|| Error::Other("no cover art found"))?;
        let query = Query::with("id", cover).arg("size", size.into()).build();

        client.get_bytes("getCoverArt", query)
    }

    fn cover_art_url<U: Into<Option<usize>>>(
        &self,
        client: &Client,
        size: U,
    ) -> Result<String> {
        let cover = self.cover_id()
            .ok_or_else(|| Error::Other("no cover art found"))?;
        let query = Query::with("id", cover).arg("size", size.into()).build();

        client.build_url("getCoverArt", query)
    }
}

impl<'de> Deserialize<'de> for ArtistInfo {
    fn deserialize<D>(de: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct _ArtistInfo {
            biography: String,
            music_brainz_id: String,
            last_fm_url: String,
            small_image_url: String,
            medium_image_url: String,
            large_image_url: String,
            similar_artist: Vec<SimilarArtist>,
        }

        let raw = _ArtistInfo::deserialize(de)?;

        Ok(ArtistInfo {
            biography: raw.biography,
            musicbrainz_id: raw.music_brainz_id,
            lastfm_url: raw.last_fm_url,
            image_urls: (
                raw.small_image_url,
                raw.medium_image_url,
                raw.large_image_url,
            ),
            similar_artists: raw.similar_artist,
        })
    }
}

impl Media for SimilarArtist {
    fn has_cover_art(&self) -> bool {
        self.cover_art.is_some()
    }

    fn cover_id(&self) -> Option<&str> {
        self.cover_art.as_ref().map(|s| s.as_str())
    }

    fn cover_art<U: Into<Option<usize>>>(&self, client: &Client, size: U) -> Result<Vec<u8>> {
        let cover = self.cover_id()
            .ok_or_else(|| Error::Other("no cover art found"))?;
        let query = Query::with("id", cover).arg("size", size.into()).build();

        client.get_bytes("getCoverArt", query)
    }

    fn cover_art_url<U: Into<Option<usize>>>(&self, client: &Client, size: U) -> Result<String> {
        let cover = self.cover_id()
            .ok_or_else(|| Error::Other("no cover art found"))?;
        let query = Query::with("id", cover).arg("size", size.into()).build();

        client.build_url("getCoverArt", query)
    }
}

impl SimilarArtist {
    /// Queries the Subsonic server to return full information about the artist.
    pub fn into_artist(self, client: &Client) -> Result<Artist> {
        self::get_artist(client, self.id)
    }
}

/// Fetches an artist from the Subsonic server.
fn get_artist(client: &Client, id: u64) -> Result<Artist> {
    let res = client.get("getArtist", Query::with("id", id))?;
    Ok(serde_json::from_value::<Artist>(res)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_util;

    #[test]
    fn parse_artist() {
        let parsed = serde_json::from_value::<Artist>(raw()).unwrap();

        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.name, String::from("Misteur Valaire"));
        assert_eq!(parsed.album_count, 1);
    }

    #[test]
    fn parse_artist_deep() {
        let parsed = serde_json::from_value::<Artist>(raw()).unwrap();

        assert_eq!(parsed.albums.len() as u64, parsed.album_count);
        assert_eq!(parsed.albums[0].id, 1);
        assert_eq!(parsed.albums[0].name, String::from("Bellevue"));
        assert_eq!(parsed.albums[0].song_count, 9);
    }

    #[test]
    fn remote_artist_album_list() {
        let mut srv = test_util::demo_site().unwrap();
        let parsed = serde_json::from_value::<Artist>(raw()).unwrap();
        let albums = parsed.albums(&mut srv).unwrap();

        assert_eq!(albums[0].id, 1);
        assert_eq!(albums[0].name, String::from("Bellevue"));
        assert_eq!(albums[0].song_count, 9);
    }

    #[test]
    fn remote_artist_cover_art() {
        let mut srv = test_util::demo_site().unwrap();
        let parsed = serde_json::from_value::<Artist>(raw()).unwrap();
        assert_eq!(parsed.cover_id, Some(String::from("ar-1")));

        let cover = parsed.cover_art(&mut srv, None).unwrap();
        assert!(!cover.is_empty())
    }

    fn raw() -> serde_json::Value {
        serde_json::from_str(
            r#"{
            "id" : "1",
            "name" : "Misteur Valaire",
            "coverArt" : "ar-1",
            "albumCount" : 1,
            "album" : [ {
                "id" : "1",
                "name" : "Bellevue",
                "artist" : "Misteur Valaire",
                "artistId" : "1",
                "coverArt" : "al-1",
                "songCount" : 9,
                "duration" : 1920,
                "playCount" : 2223,
                "created" : "2017-03-12T11:07:25.000Z",
                "genre" : "(255)"
            } ]
        }"#,
        ).unwrap()
    }

}