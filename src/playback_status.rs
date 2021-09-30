use rspotify::spotify::model::context::SimplifiedPlayingContext;
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Default)]
pub struct PlaybackStatus {
    is_playing: bool,
    progress_ms: Option<u32>,
    duration_ms: Option<u32>,
    track: Option<String>,
    artists: Option<Vec<String>>,
    album: Option<String>,
}

impl From<SimplifiedPlayingContext> for PlaybackStatus {
    fn from(status: SimplifiedPlayingContext) -> Self {
        let is_playing = status.is_playing;
        let progress_ms = status.progress_ms;
        let fulltrack = status.item.as_ref();
        let duration_ms = fulltrack.map(|track| track.duration_ms);
        let track = fulltrack
            .map(|t| t.name.to_owned())
            .or_else(|| Some(String::from("Unknown")));
        let artists = fulltrack
            .map(|t| {
                t.artists
                    .iter()
                    .map(|a| a.name.to_owned())
                    .collect::<Vec<String>>()
            })
            .or_else(|| Some(vec![String::from("Unknown")]));
        let album = fulltrack
            .map(|t| t.album.name.to_owned())
            .or_else(|| Some(String::from("Unknown")));
        PlaybackStatus {
            is_playing,
            progress_ms,
            duration_ms,
            track,
            album,
            artists,
        }
    }
}

impl fmt::Display for PlaybackStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "{}", serde_json::to_string(self).unwrap());
        Ok(())
    }
}
