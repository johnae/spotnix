use rspotify::spotify::model::album::SimplifiedAlbum;
use rspotify::spotify::model::artist::FullArtist;
use rspotify::spotify::model::device::Device;
use rspotify::spotify::model::playlist::SimplifiedPlaylist;
use rspotify::spotify::model::track::FullTrack;
use std::fmt;

pub enum Output {
    SearchTracks(Vec<FullTrack>),
    SearchAlbums(Vec<SimplifiedAlbum>),
    SearchArtists(Vec<FullArtist>),
    SearchPlaylists(Vec<SimplifiedPlaylist>),
    Devices(Vec<Device>),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SearchTracks(tracks) => {
                let mut buf = String::with_capacity(500);
                for item in tracks.iter() {
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
            Self::SearchAlbums(albums) => {
                let mut buf = String::with_capacity(100);
                for item in albums.iter() {
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
            Self::SearchArtists(artists) => {
                let mut buf = String::with_capacity(100);
                for item in artists.iter() {
                    buf.push_str(&format!("{} - {}\n", item.name, item.uri));
                }
                write!(f, "{}", buf)
            }
            Self::SearchPlaylists(playlists) => {
                let mut buf = String::with_capacity(100);
                for item in playlists.iter() {
                    buf.push_str(&format!("{} - {}\n", item.name, item.uri));
                }
                write!(f, "{}", buf)
            }
            Self::Devices(devices) => {
                let mut buf = String::with_capacity(50);
                for item in devices.iter() {
                    buf.push_str(&format!("{} - {}\n", item.name, item.id));
                }
                write!(f, "{}", buf)
            }
        }
    }
}
