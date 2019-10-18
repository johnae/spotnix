use rspotify::spotify::model::context::SimplifiedPlayingContext;
use std::fmt;

pub enum Event {
    PlaybackStatus {
        progress_ms: u32,
        duration_ms: u32,
        track: String,
        artists: String,
        album: String,
    },
}

impl From<SimplifiedPlayingContext> for Event {
    fn from(status: SimplifiedPlayingContext) -> Self {
        let progress_ms = status.progress_ms.or(Some(0)).unwrap();
        let fulltrack = status.item.as_ref();
        let duration_ms = fulltrack
            .and_then(|track| Some(track.duration_ms))
            .or(Some(0))
            .unwrap();
        let track = fulltrack
            .and_then(|t| Some(t.name.clone()))
            .or(Some(String::from("Unknown")))
            .unwrap();
        let artists = fulltrack
            .and_then(|t| {
                Some(
                    t.artists
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect::<Vec<&str>>()
                        .join(", "),
                )
            })
            .or(Some(String::from("Unknown")))
            .unwrap();
        let album = fulltrack
            .and_then(|t| Some(t.album.name.clone()))
            .or(Some(String::from("Unknown")))
            .unwrap();
        Self::PlaybackStatus {
            progress_ms,
            duration_ms,
            track,
            album,
            artists,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlaybackStatus {
                progress_ms,
                duration_ms,
                track,
                artists,
                album,
            } => write!(
                f,
                "PlaybackStatus #track:{} #album:{} #artists:{} #progress_ms:{} #duration_ms:{}",
                track, album, artists, progress_ms, duration_ms
            ),
        }
    }
}
