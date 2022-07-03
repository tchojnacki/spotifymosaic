"""A module used to generate cover for Spotify playlist based on album covers"""
import base64
import re
import math
import random
import logging
import sys
from multiprocessing.dummy import Pool
import requests
from PIL import Image

class SpotifyMosaic:
    """Main class"""

    def __init__(self, client):
        """Constructor"""
        self._token = None
        self.spotifyid = client["id"]
        self.spotifysecret = client["secret"]

    def get_albums(self, playlist):
        """Get albums from a playlist"""
        logging.info("Fetching albums from playlist.")
        headers = {
            "Authorization": self.token
        }
        album_ids = []
        next_url = SpotifyMosaic.uri_to_api(playlist)
        while next_url is not None:
            logging.info("Sending request to playlist API.")
            r = requests.get(next_url, headers=headers)
            if r.status_code != 200:
                logging.critical("Playlist not found.")
                sys.exit()
            album_ids = [*album_ids, *[item["track"]["album"]["id"] for item in r.json()["items"]]]
            next_url = r.json()["next"]
        logging.info("Album ids obtained.")
        return list(dict.fromkeys(album_ids))

    def get_artworks(self, album_ids, resolution=640):
        """Get artworks for a list of albums"""
        logging.info("Fetching artworks from albums.")
        url = "https://api.spotify.com/v1/albums/"
        headers = {
            "Authorization": self.token
        }
        resolution_dict = dict([(640, 0), (300, 1), (64, 2)])
        resolution = resolution_dict[resolution]
        album_ids = [album_ids[i:i + 20] for i in range(0, len(album_ids), 20)]
        images = []
        for chunk in album_ids:
            logging.info("Sending request to album API.")
            r = requests.get(url, headers=headers, params={"ids": ",".join(chunk)})
            images = [*images, *[album["images"][resolution]["url"] for album in r.json()["albums"]]]
        logging.info("Album artworks obtained.")
        return images

    def generate_mosaic(self, artworks, size=2, output="mosaic.jpg", shuffle=False, resolution=640):
        """Generate a mosaic based on artworks list"""
        logging.info("Generating mosaic.")
        if shuffle is True:
            random.shuffle(artworks)
        max_tiles = math.floor(math.sqrt(len(artworks)))
        tiles = min(size, max_tiles)
        artworks = artworks[:tiles**2]

        logging.info("Requesting images.")

        with Pool(16) as p:
            images = p.map(lambda artwork: Image.open(requests.get(artwork, stream=True).raw), artworks)

        logging.info("Creating image.")
        new_image = Image.new("RGB", (tiles * resolution, tiles * resolution))
        current_image = 0
        for x in range(tiles):
            for y in range(tiles):
                try:
                    new_image.paste(images[current_image], (x * resolution, y * resolution))
                except MemoryError:
                    logging.critical("Python ran out of memory while creating the image.")
                    raise
                current_image += 1
        if output is None:
            return new_image
        logging.info("Saving the image.")
        new_image.save(output)
        return output

    def create(self, playlist, size=2, output="mosaic.jpg", shuffle=False, resolution=640):
        """Generate a mosaic from playlist"""
        albums = self.get_albums(playlist)
        artworks = self.get_artworks(albums, resolution=resolution)
        return self.generate_mosaic(artworks, size=size, output=output, shuffle=shuffle, resolution=resolution)

    @staticmethod
    def uri_to_api(uri):
        """Generate link to playlist endpoint based on Spotify Playlist URI"""
        m = re.match(r"^spotify:user:(.+?):playlist:(.+?)$", uri)
        if m is None:
            m = re.match(r"^https?:\/\/open\.spotify\.com\/user\/(.+?)\/playlist\/(.+?)($|\?)", uri)
            if m is None:
                logging.critical("Incorrect playlist URI.")
                sys.exit()
        return "https://api.spotify.com/v1/users/{}/playlists/{}/tracks".format(m.group(1), m.group(2))

    @property
    def token(self):
        """Get token Bearer"""
        logging.info("Requesting token.")
        if self._token is not None:
            return self._token

        logging.info("Generating token.")
        url = "https://accounts.spotify.com/api/token"
        headers = {
            "Authorization": "Basic " + base64.b64encode(("{}:{}".format(self.spotifyid, self.spotifysecret)).encode("utf-8")).decode("utf-8")
        }
        data = {
            "grant_type": "client_credentials"
        }
        r = requests.post(url, headers=headers, data=data)

        if r.status_code != 200:
            logging.critical("Token not found. Wrong Spotify API ID or Secret.")
            sys.exit()
        logging.info("Authorization complete.")
        self._token = "{} {}".format(r.json()["token_type"], r.json()["access_token"])
        return self._token

if __name__ == "__main__":
    logging.basicConfig(format="%(levelname)s %(asctime)s: %(message)s", datefmt="%H:%M:%S")
    logging.critical("Please use mosaic.py for commandline use.")
