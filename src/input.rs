use crate::Result;
use crate::SpotnixError;
use std::fmt;
use std::str::FromStr;

pub enum SpotifyUri {
    TrackUri(String),
    AlbumUri(String),
    ArtistUri(String),
    PlaylistUri(String),
}

impl fmt::Display for SpotifyUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrackUri(id) => write!(f, "{}", id),
            Self::AlbumUri(id) => write!(f, "{}", id),
            Self::ArtistUri(id) => write!(f, "{}", id),
            Self::PlaylistUri(id) => write!(f, "{}", id),
        }
    }
}

impl FromStr for SpotifyUri {
    type Err = SpotnixError;
    fn from_str(s: &str) -> Result<Self> {
        let mut uri_parts = s.split(':');
        match uri_parts.next() {
            Some("spotify") => match uri_parts.next() {
                Some("track") => {
                    if uri_parts.next().is_some() {
                        return Ok(SpotifyUri::TrackUri(s.to_string()));
                    }
                    Err(SpotnixError::Parse(format!("bad spotify uri: {}", s)))
                }
                Some("album") => {
                    if uri_parts.next().is_some() {
                        return Ok(SpotifyUri::AlbumUri(s.to_string()));
                    }
                    Err(SpotnixError::Parse(format!("bad spotify uri: {}", s)))
                }
                Some("artist") => {
                    if uri_parts.next().is_some() {
                        return Ok(SpotifyUri::ArtistUri(s.to_string()));
                    }
                    Err(SpotnixError::Parse(format!("bad spotify uri: {}", s)))
                }
                Some("playlist") => {
                    if uri_parts.next().is_some() {
                        return Ok(SpotifyUri::PlaylistUri(s.to_string()));
                    }
                    Err(SpotnixError::Parse(format!("bad spotify uri: {}", s)))
                }
                _ => Err(SpotnixError::Parse(format!("bad id string: {}", s))),
            },
            _ => Err(SpotnixError::Parse(format!("bad id string: {}", s))),
        }
    }
}

pub enum Input {
    PlayTrack(String),
    PlayAlbum(String),
    PlayArtist(String),
    PlayPlaylist(String),
    Play,
    PlaybackStatus(u32, u32),
    SearchTrack(String),
    SearchAlbum(String),
    SearchArtist(String),
    SearchPlaylist(String),
    Device(String),
    ListDevices,
    Stop,
    Pause,
    Next,
    Previous,
    TokenRefresh,
    Shuffle(bool),
}

impl FromStr for Input {
    type Err = SpotnixError;
    fn from_str(s: &str) -> Result<Self> {
        let mut cmd_parts = s.split_whitespace();
        match cmd_parts.next() {
            Some("play") | Some("p") => match cmd_parts.next() {
                Some(part) => {
                    let uri: SpotifyUri = part.parse()?;
                    match uri {
                        SpotifyUri::TrackUri(uri) => Ok(Input::PlayTrack(uri)),
                        SpotifyUri::AlbumUri(uri) => Ok(Input::PlayAlbum(uri)),
                        SpotifyUri::ArtistUri(uri) => Ok(Input::PlayArtist(uri)),
                        SpotifyUri::PlaylistUri(uri) => Ok(Input::PlayPlaylist(uri)),
                    }
                }
                _ => Ok(Input::Play),
            },
            Some("shuffle") => match cmd_parts.next() {
                Some(part) => Ok(Input::Shuffle(part.parse()?)),
                _ => Err(SpotnixError::Parse(String::from("shuffle takes a bool"))),
            },
            Some("search_track") | Some("s") => {
                let what = cmd_parts.collect::<Vec<&str>>().join(" ");
                if what.is_empty() {
                    return Err(SpotnixError::Parse(String::from(
                        "empty search string not allowed",
                    )));
                }
                Ok(Input::SearchTrack(what))
            }
            Some("search_album") | Some("sab") => {
                let what = cmd_parts.collect::<Vec<&str>>().join(" ");
                if what.is_empty() {
                    return Err(SpotnixError::Parse(String::from(
                        "empty search string not allowed",
                    )));
                }
                Ok(Input::SearchAlbum(what))
            }
            Some("search_artist") | Some("sar") => {
                let what = cmd_parts.collect::<Vec<&str>>().join(" ");
                if what.is_empty() {
                    return Err(SpotnixError::Parse(String::from(
                        "empty search string not allowed",
                    )));
                }
                Ok(Input::SearchArtist(what))
            }
            Some("search_playlist") | Some("sap") => {
                let what = cmd_parts.collect::<Vec<&str>>().join(" ");
                if what.is_empty() {
                    return Err(SpotnixError::Parse(String::from(
                        "empty search string not allowed",
                    )));
                }
                Ok(Input::SearchPlaylist(what))
            }
            Some("devices") | Some("list_devices") | Some("ld") => Ok(Input::ListDevices),
            Some("device") | Some("d") => {
                let what = cmd_parts.next().map(|s| s.to_string());
                if let Some(id) = what {
                    return Ok(Input::Device(id));
                }
                Err(SpotnixError::Parse(String::from("device requires an id")))
            }
            Some("stop") => Ok(Input::Stop),
            Some("pause") => Ok(Input::Pause),
            Some("next") => Ok(Input::Next),
            Some("prev") | Some("previous") => Ok(Input::Previous),
            _ => Err(SpotnixError::Parse(String::from(
                "don't know what you mean",
            ))),
        }
    }
}
