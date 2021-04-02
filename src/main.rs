use log::{debug, error, info};
use rayon::prelude::*;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::model::artist::SimplifiedArtist;
use rspotify::spotify::model::context::SimplifiedPlayingContext;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::senum::Country;
use rspotify::spotify::util::get_token;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use std::{thread, time};
use structopt::StructOpt;
use timer::Timer;

mod errors;
use errors::SpotnixError;

mod input;
use input::Input;

mod output;
use output::Output;

mod event;
use event::Event;

use nix::sys::stat;
use nix::unistd;

type Result<T> = std::result::Result<T, errors::SpotnixError>;

#[derive(StructOpt)]
#[structopt(
    name = "spotnix",
    about = "spotify as a series of \"tubes\"... or rather named pipes."
)]
struct Opt {
    /// All output goes here (a named pipe)
    #[structopt(short = "o", long = "output", default_value = "./output")]
    output: String,
    /// All input goes here (a named pipe)
    #[structopt(short = "i", long = "input", default_value = "./input")]
    input: String,
    /// All events go here (a named pipe)
    #[structopt(short = "e", long = "event", default_value = "./event")]
    event: String,
    /// Default device to use by name
    #[structopt(short = "d", long = "device")]
    device: Option<String>,
    /// Max number of search result pages to fetch (a page contains 50 items)
    #[structopt(short = "r", long = "pages", default_value = "4")]
    pages: u32,
}

const SCOPES: [&str; 9] = [
    "playlist-read-private",
    "user-follow-read",
    "user-library-modify",
    "user-library-read",
    "user-modify-playback-state",
    "user-read-currently-playing",
    "user-read-playback-state",
    "user-read-private",
    "user-read-recently-played",
];

struct Spotnix {
    spotify: Spotify,
    token_expiry: Instant,
    device: Option<String>,
    event: String,
    event_tx: Sender<Event>,
    input: String,
    input_rx: Receiver<Input>,
    output: String,
    output_tx: Sender<Output>,
    status: Option<SimplifiedPlayingContext>,
    max_pages: u32,
}

fn new_spotify_client() -> Result<(Spotify, Instant)> {
    let client_id = env::var("CLIENT_ID").unwrap_or_default();
    let client_secret = env::var("CLIENT_SECRET").unwrap_or_default();
    let redirect_uri = env::var("REDIRECT_URI").unwrap_or_default();

    info!("using client_id: {}", client_id);

    let mut oauth = SpotifyOAuth::default()
        .client_id(&client_id)
        .client_secret(&client_secret)
        .redirect_uri(&redirect_uri)
        .scope(&SCOPES.join(" "))
        .build();

    let token_info = get_token(&mut oauth).ok_or("No token")?;
    let token_expiry = Instant::now() + Duration::from_secs(token_info.expires_in.into())
        - Duration::from_secs(120);
    info!(
        "token will expire in {:?}",
        Duration::from_secs(token_info.expires_in.into())
    );

    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    Ok((spotify, token_expiry))
}

impl Spotnix {
    fn new<S>(
        input: S,
        output: S,
        event: S,
        event_tx: Sender<Event>,
        input_rx: Receiver<Input>,
        output_tx: Sender<Output>,
        max_pages: u32,
    ) -> Result<Self>
    where
        S: Into<String>,
    {
        let mkpipe = |name: &str| {
            if let Err(e) = std::fs::remove_file(name) {
                debug!("couldn't remove named pipe '{}': {}", name, e);
            }
            match unistd::mkfifo(name, stat::Mode::S_IRWXU) {
                Ok(_) => println!("created fifo {}", name),
                Err(err) => {
                    if err.as_errno() == Some(nix::errno::Errno::EEXIST) {
                        println!("{} pipe already present", name);
                    } else {
                        panic!("couldn't create named pipe {}", name);
                    }
                }
            };
        };

        let (spotify, token_expiry) = new_spotify_client()?;

        let spotnix = Self {
            spotify,
            token_expiry,
            event: event.into(),
            event_tx,
            input: input.into(),
            input_rx,
            output: output.into(),
            output_tx,
            device: None,
            status: None,
            max_pages,
        };

        mkpipe(&spotnix.input);
        mkpipe(&spotnix.output);
        mkpipe(&spotnix.event);

        Ok(spotnix)
    }

    fn run(&mut self) -> Result<()> {
        loop {
            if let Ok(cmd) = self.input_rx.recv() {
                self.exec(cmd)?;
            }
        }
    }

    fn maybe_refresh_token(&mut self) -> Result<()> {
        if time::Instant::now() > self.token_expiry {
            info!("refreshing auth token");

            let (spotify, token_expiry) = new_spotify_client()?;
            self.spotify = spotify;
            self.token_expiry = token_expiry;
        }
        Ok(())
    }

    fn exec(&mut self, input: Input) -> Result<()> {
        let get_artists = |artists: &Vec<SimplifiedArtist>| -> String {
            artists
                .iter()
                .map(|artist| artist.name.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        };
        match input {
            Input::TokenRefresh => self.maybe_refresh_token()?,
            Input::Play => {
                self.spotify
                    .start_playback(self.device.clone(), None, None, None, None)?;
            }
            Input::PlayTrack(track_id) => {
                let track = self.spotify.track(&track_id)?;
                let artists_str = get_artists(&track.artists);
                info!(
                    "play track id: {}, name: {}, artists: {}",
                    track_id, track.name, artists_str
                );
                let album = track.album.clone();
                let mut tracks = vec![track_id];
                if let Some(album_id) = album.id {
                    let album = self.spotify.album(&album_id)?;
                    let more_uris: &mut Vec<String> = &mut album
                        .tracks
                        .items
                        .into_iter()
                        .filter(|ref t| t.uri != track.uri)
                        .map(|t| t.uri)
                        .collect();
                    tracks.append(more_uris);
                }
                self.spotify
                    .start_playback(self.device.clone(), None, Some(tracks), None, None)?;
            }
            Input::PlayAlbum(album_id) => {
                let album = self.spotify.album(&album_id)?;
                let artists_str = get_artists(&album.artists);
                info!(
                    "play album id: {}, name: {}, artists: {}",
                    album_id, album.name, artists_str
                );
                self.spotify.start_playback(
                    self.device.clone(),
                    Some(album_id),
                    None,
                    None,
                    None,
                )?;
            }
            Input::PlayArtist(artist_id) => {
                let artist = self.spotify.artist(&artist_id)?;
                info!("play artist id: {}, name: {}", artist_id, artist.name);
                self.spotify.start_playback(
                    self.device.clone(),
                    Some(artist_id),
                    None,
                    None,
                    None,
                )?;
            }
            Input::PlayPlaylist(playlist_id) => {
                let playlist = self.spotify.playlist(&playlist_id, None, None)?;
                info!("play playlist id: {}, name: {}", playlist_id, playlist.name);
                self.spotify.start_playback(
                    self.device.clone(),
                    Some(playlist_id),
                    None,
                    None,
                    None,
                )?;
            }
            Input::Shuffle(state) => {
                self.spotify.shuffle(state, self.device.clone())?;
            }
            Input::PlaybackStatus(skew, update) => {
                let status = if skew == update {
                    self.status = self.spotify.current_playing(Some(Country::Sweden))?;
                    self.status.clone()
                } else {
                    let mut status = self.status.clone();
                    if let Some(ref mut status) = status {
                        if status.is_playing {
                            status.progress_ms = status.progress_ms.map(|v| v + (1000 * skew));
                        };
                    };
                    status
                };
                if let Some(status) = status {
                    self.event_tx.send(status.into())?;
                };
            }
            Input::SearchTrack(search) => {
                let mut results = vec![];
                let spotify = &self.spotify;
                results.par_extend((1..=self.max_pages).into_par_iter().flat_map(|i| {
                    if let Ok(stracks) = spotify.search_track(
                        search.as_str(),
                        50,
                        50 * (i - 1),
                        Some(Country::Sweden),
                    ) {
                        stracks.tracks.items
                    } else {
                        vec![]
                    }
                }));
                self.output_tx.send(Output::SearchTracks(results))?;
            }
            Input::SearchAlbum(search) => {
                let mut results = vec![];
                let spotify = &self.spotify;
                results.par_extend((1..=self.max_pages).into_par_iter().flat_map(|i| {
                    if let Ok(salbums) = spotify.search_album(
                        search.as_str(),
                        50,
                        50 * (i - 1),
                        Some(Country::Sweden),
                    ) {
                        salbums.albums.items
                    } else {
                        vec![]
                    }
                }));
                self.output_tx.send(Output::SearchAlbums(results))?;
            }
            Input::SearchArtist(search) => {
                let mut results = vec![];
                let spotify = &self.spotify;
                results.par_extend((1..=self.max_pages).into_par_iter().flat_map(|i| {
                    if let Ok(salbums) = spotify.search_artist(
                        search.as_str(),
                        50,
                        50 * (i - 1),
                        Some(Country::Sweden),
                    ) {
                        salbums.artists.items
                    } else {
                        vec![]
                    }
                }));
                self.output_tx.send(Output::SearchArtists(results))?;
            }
            Input::SearchPlaylist(search) => {
                let mut results = vec![];
                let spotify = &self.spotify;
                results.par_extend((1..=self.max_pages).into_par_iter().flat_map(|i| {
                    if let Ok(splaylists) = spotify.search_playlist(
                        search.as_str(),
                        50,
                        50 * (i - 1),
                        Some(Country::Sweden),
                    ) {
                        splaylists.playlists.items
                    } else {
                        vec![]
                    }
                }));
                self.output_tx.send(Output::SearchPlaylists(results))?;
            }
            Input::Device(id) => {
                self.spotify.transfer_playback(&id, false)?;
                self.device = Some(id);
            }
            Input::ListDevices => {
                let results = self.spotify.device()?;
                self.output_tx.send(Output::Devices(results.devices))?;
            }
            Input::Pause | Input::Stop => {
                self.spotify.pause_playback(self.device.clone())?;
            }
            Input::Next => {
                self.spotify.next_track(self.device.clone())?;
            }
            Input::Previous => {
                self.spotify.previous_track(self.device.clone())?;
            }
        };
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    let (input_tx, input_rx): (Sender<Input>, Receiver<Input>) = mpsc::channel();
    let (output_tx, output_rx): (Sender<Output>, Receiver<Output>) = mpsc::channel();
    let (event_tx, event_rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    let mut spotnix = Spotnix::new(
        opt.input, opt.output, opt.event, event_tx, input_rx, output_tx, opt.pages,
    )?;

    if let Some(device_name) = opt.device {
        let device = spotnix
            .spotify
            .device()?
            .devices
            .into_iter()
            .find(|device| device.name == device_name)
            .unwrap_or_else(|| panic!("device '{}' not found", device_name))
            .id;
        info!("using device: {}", device);
        spotnix.device = Some(device);
    } else {
        info!("no device given on cmdline, you must select one before you can play music");
    }

    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&spotnix.input)?;

    let in_tx = input_tx.clone();
    thread::spawn(move || loop {
        let file = BufReader::new(&f);
        for line in file.lines() {
            let line = line.expect("line from named pipe not there");
            info!("read input: '{}'", line);
            match line.parse::<Input>() {
                Ok(input) => {
                    let _ = in_tx.send(input);
                }
                Err(err) => {
                    error!("failed to parse input: '{}' - {}", line, err);
                }
            };
        }
    });

    let output = spotnix.output.clone();
    thread::spawn(move || -> Result<()> {
        loop {
            if let Ok(out) = output_rx.recv() {
                let f = OpenOptions::new()
                    .read(false)
                    .write(true)
                    .open(output.as_str())
                    .expect("file not found");
                let mut bw = BufWriter::new(f);
                let res = bw.write_fmt(format_args!("{}", out.to_string()));
                if let Err(err) = res {
                    info!("failed to write to output: {}", err);
                }
            }
        }
    });

    let event = spotnix.event.clone();
    thread::spawn(move || -> Result<()> {
        loop {
            if let Ok(out) = event_rx.recv() {
                let f = OpenOptions::new()
                    .read(true) // keep it open
                    .write(true)
                    .open(event.as_str())
                    .expect("file not found");
                let mut bw = BufWriter::new(f);
                let res = bw.write_fmt(format_args!("{}", out.to_string()));
                if let Err(err) = res {
                    info!("failed to write to output: {}", err);
                }
            }
        }
    });

    let timer = Timer::new();
    let _guard = {
        let in_tx = input_tx;
        let mut count = 5;
        timer.schedule_repeating(chrono::Duration::seconds(1), move || {
            let _ = in_tx.send(Input::TokenRefresh);
            let _ = in_tx.send(Input::PlaybackStatus(count, 5));
            if count < 5 {
                count += 1;
            } else {
                count = 0;
            }
        })
    };

    spotnix.run()?;
    Ok(())
}
