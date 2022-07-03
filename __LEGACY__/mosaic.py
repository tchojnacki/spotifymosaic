"""CMD utility for SpotifyMosaic"""
import argparse
import logging
from main import SpotifyMosaic

def main():
    """Called after using python mosaic.py"""
    parser = argparse.ArgumentParser(description="Generate a mosaic from spotify playlist.")

    parser.add_argument("spotifyid", help="Spotify API Client ID")
    parser.add_argument("spotifysecret", help="Spotify API Client Secret")
    parser.add_argument("playlist", help="Spotify playlist URI")

    parser.add_argument("-t", "--tiles", help="number of artworks per mosaic row",
                        type=int, default=2)
    parser.add_argument("-o", "--out", help="output file",
                        default="mosaic.jpg")
    parser.add_argument("-s", "--shuffle", help="randomize the order of artworks",
                        action="store_true")
    parser.add_argument("-r", "--resolution", help="select the resolution of one artwork",
                        type=int, default=640, choices=[64, 300, 640])
    parser.add_argument("-l", "--log", help="enable logging",
                        action="store_true")

    args = parser.parse_args()

    if args.log:
        logging.basicConfig(level=logging.INFO, format="%(levelname)s %(asctime)s: %(message)s", datefmt="%H:%M:%S")
    else:
        logging.basicConfig(level=logging.CRITICAL, format="%(levelname)s %(asctime)s: %(message)s", datefmt="%H:%M:%S")

    logging.info("Creating class instance.")
    mosaic = SpotifyMosaic({
        "id": args.spotifyid,
        "secret": args.spotifysecret
    })

    logging.info("Running main function.")
    mosaic.create(args.playlist, size=args.tiles, output=args.out, shuffle=args.shuffle, resolution=args.resolution)

if __name__ == "__main__":
    main()
