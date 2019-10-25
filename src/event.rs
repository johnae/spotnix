use rspotify::spotify::model::context::SimplifiedPlayingContext;
use serde::Serialize;
use std::fmt;

// it IS constructed
#[allow(dead_code)]
#[derive(Serialize)]
pub enum Event {
    PlaybackStatus {
        progress_ms: u32,
        duration_ms: u32,
        track: String,
        artists: Vec<String>,
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
            .and_then(|t| Some(t.name.to_owned()))
            .or(Some(String::from("Unknown")))
            .unwrap();
        let artists = fulltrack
            .and_then(|t| {
                Some(
                    t.artists
                        .iter()
                        .map(|a| a.name.to_owned())
                        .collect::<Vec<String>>(),
                )
            })
            .or(Some(vec![String::from("Unknown")]))
            .unwrap();
        let album = fulltrack
            .and_then(|t| Some(t.album.name.to_owned()))
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
        let _ = write!(f, "{}", serde_json::to_string(self).unwrap());
        Ok(())
    }
}
