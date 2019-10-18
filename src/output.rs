use rspotify::spotify::model::device::Device;
use rspotify::spotify::model::search::{
    SearchAlbums, SearchArtists, SearchPlaylists, SearchTracks,
};
use std::fmt;

pub enum Output {
    SearchTracks(SearchTracks),
    SearchAlbums(SearchAlbums),
    SearchArtists(SearchArtists),
    SearchPlaylists(SearchPlaylists),
    Devices(Vec<Device>),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SearchTracks(trackslist) => {
                let mut buf = String::with_capacity(500);
                for item in trackslist.tracks.items.iter() {
                    let artists = item
                        .artists
                        .iter()
                        .map(|artist| artist.name.as_str())
                        .collect::<Vec<&str>>()
                        .join(", ");
                    buf.push_str(&format!("{} - {} - {}\n", artists, item.name, item.uri));
                }
                write!(f, "{}", buf)
            }
            Self::SearchAlbums(albumlist) => {
                let mut buf = String::with_capacity(100);
                for item in albumlist.albums.items.iter() {
                    let artists = item
                        .artists
                        .iter()
                        .map(|artist| artist.name.as_str())
                        .collect::<Vec<&str>>()
                        .join(", ");
                    if let Some(uri) = &item.uri {
                        buf.push_str(&format!("{} - {} - {}\n", artists, item.name, uri));
                    } else {
                        buf.push_str(&format!("{} - {} - none\n", artists, item.name));
                    }
                }
                write!(f, "{}", buf)
            }
            Self::SearchArtists(artistslist) => {
                let mut buf = String::with_capacity(100);
                for item in artistslist.artists.items.iter() {
                    buf.push_str(&format!("{} - {}\n", item.name, item.uri));
                }
                write!(f, "{}", buf)
            }
            Self::SearchPlaylists(playlistslist) => {
                let mut buf = String::with_capacity(100);
                for item in playlistslist.playlists.items.iter() {
                    buf.push_str(&format!("{} - {}\n", item.name, item.uri));
                }
                write!(f, "{}", buf)
            }
            Self::Devices(devicelist) => {
                let mut buf = String::with_capacity(50);
                for item in devicelist.iter() {
                    buf.push_str(&format!("{} - {}\n", item.name, item.id));
                }
                write!(f, "{}", buf)
            }
        }
    }
}
