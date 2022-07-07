# spotifymosaic
Generate a mosaic for a given Spotify playlist using album artworks.

## Installation
```
git clone https://github.com/tchojnacki/spotifymosaic.git
cd spotifymosaic
cargo install --path .
```

## Usage
```
USAGE:
    spotifymosaic [OPTIONS] <--creds <CLIENT_ID:CLIENT_SECRET>> <PLAYLIST_URI>

ARGS:
    <PLAYLIST_URI>    Spotify playlist URI

OPTIONS:
        --creds <CLIENT_ID:CLIENT_SECRET>
            Spotify client's ID and secret delimited by a colon

    -t, --tiles <TILE_SIDE_LEN>
            Mosaic's side length [default: 2]

    -o, --out <OUTPUT_PATH>
            Output image file path [default: mosaic.png]

    -a, --arrange <ARRANGEMENT>
            Order of mosaic's squares [default: first] [possible values: first, last, random]

    -r, --res <RESOLUTION>
            Output image's resolution [default: 640]

    -h, --help
            Print help information
```
