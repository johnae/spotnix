### Spotify for Unix

What does that even mean? It means that this is how I like to search for music and listen to music on a Unix system. This is a spotify controller and does not (yet anyway) include a way to stream the actual music. But there's already a very good such player (which I use with Spotnix): https://github.com/spotifyd/spotifyd.

_It's really really early days so there's likely to be some rough edges, missing features and scrappyness... however, I use it daily._

First, here's a taste of what a session might look like (spotnix on the left, on the right we search and select what to play):

![spotnix](https://user-images.githubusercontent.com/28332/67158210-93343800-f335-11e9-93fc-45cafd964a8b.gif)


So what's so special about this then? Well, you get three named pipes and that's it. What's not to like :-)? The standard names of those pipes:

`./input` (eg. play something, pause, list devices, search etc.)

`./output` (results of a search, results of list devices etc.)

`./status` (playback status such as progress, track name etc). This is json.

```sh
echo search_playlist electronic > ./input
```

To get the results and select something to play (you can ofc use https://github.com/junegunn/fzf as well, I like https://github.com/lotabout/skim - probably just because I'm biased as I'm very fond of the Rust programming language):

```sh
cat ./output | sk | awk '{print $NF}' | xargs -r -I{} echo play {} > ./input
```

Or just all in one:

```sh
echo search_playlist electronic > ./input; cat ./output | sk | awk '{print $NF}' | xargs -r -I{} echo play {} > ./input
```

To get playback status (eg. progress, name of song / album etc):

```sh
cat ./status
```

(The above updates every 1 second. Just `cat` again or read that pipe in a loop or whatever you like for updates.)

Obviously you'd probably put all the above things in scripts - but it's up to you how you use it, these are just building blocks - the Unix way. For some basic example scripts, see: [examples](examples/)


The available commands at the moment are (those you send into the ./input pipe):

```sh
play <optional spotify uri> (without uri - resume playback)
pause
shuffle true/false
search_track <search string>
search_album <search string>
search_artist <search string>
search_playlist <search string>
devices (list devices available)
device <device id> (selects the device use for playback)
next
prev
```

The search results are returned in pages with the spotify enforced maximum of 50 items. Since this tool is meant to be used on the command line and actual selection of what to play will likely happen from a newline delimited list of results (using tools like above mentioned `fzf` or `skim`), there's a command line option for setting the number of pages to fetch (default is 4). These pages are fetched concurrently using the awesome `rayon` crate.

### Installing

At the moment there's no packages so you'll have to build it. First, install [Rust](https://www.rust-lang.org/tools/install) then

```bash
cargo install spotnix
```

Personally I use [Nix](https://nixos.org/nix) to manage dependencies, including the Rust toolchain, but I know [Rustup](https://rustup.rs/) is really good. You do need *openssl* installed for compilation.

To update spotnix you can run

```bash
cargo install spotnix --force
```

## To use this program, you must connect to Spotify's Web API

`spotnix` talks to Spotify's Web API to do it's thing (eg. search, play, pause etc.).

When you start `spotnix` the first time, you will be asked to login to spotify BUT - to really make it work, you must actually do a little bit more:

1. Open the [Spotify applications dashboard](https://developer.spotify.com/dashboard/applications)
2. Click `Create a Client ID` and create an app
3. Click `Edit Settings`
4. Add `http://localhost:8182/spotnix` to the Redirect URIs
5. Now you can authenticate with Spotify!
6. So back in the terminal, run `CLIENT_ID=abcde12345 CLIENT_SECRET=sasjfijs2983 REDIRECT_URI=http://localhost:8182/spotnix`
7. Now you'll be redirected - in your browser - to Spotify which will ask you to grant permissions.
8. After accepting, you'll be redirected to localhost which will very likely refuse the connection (unless you really have a web server running on port 8182). Either way, copy the URL and paste into the the terminal where it asks for it (you may need to close the browser window before it asks you for it in the terminal).


### Usage

This is currently the help output of spotnix:

```sh
spotnix 0.1.2
spotify as a series of "tubes"... or rather named pipes.

USAGE:
    spotnix [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --device <device>    Default device to use by name
    -s, --status <status>      All events go here (a named pipe) [default: ./status]
    -i, --input <input>      All input goes here (a named pipe) [default: ./input]
    -o, --output <output>    All output goes here (a named pipe) [default: ./output]
    -r, --pages <pages>      Max number of search result pages to fetch (a page contains 50 items) [default: 4]
```

So that is Spotify for Unix - you get to use all those command line tools you know and love to find and listen to music!
