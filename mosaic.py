"""A module used to generate cover for Spotify playlist based on album covers"""
import base64
import re
import math
import urllib.request
import random
import argparse
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
        headers = {
            "Authorization": self.token
        }
        album_ids = []
        next_url = SpotifyMosaic.uri_to_api(playlist)
        while next_url is not None:
            r = requests.get(next_url, headers=headers).json()
            album_ids = [*album_ids, *[item["track"]["album"]["id"] for item in r["items"]]]
            next_url = r["next"]
        return list(dict.fromkeys(album_ids))

    def get_artworks(self, album_ids):
        """Get artworks for a list of albums"""
        url = "https://api.spotify.com/v1/albums/"
        headers = {
            "Authorization": self.token
        }
        album_ids = [album_ids[i:i + 20] for i in range(0, len(album_ids), 20)]
        images = []
        for chunk in album_ids:
            r = requests.get(url, headers=headers, params={"ids": ",".join(chunk)})
            images = [*images, *[album["images"][0]["url"] for album in r.json()["albums"]]]
        return images

    def download_artworks(self, artworks, directory="images/"):
        """Download artworks from urls"""
        images = []
        for index, artwork in enumerate(artworks):
            urllib.request.urlretrieve(artwork, (directory + "{}.png").format(index))
            images.append((directory + "{}.png").format(index))
        return {
            "directory": directory,
            "length": len(images),
            "images": images
        }

    def generate_mosaic(self, artworks, size=2, output="mosaic.png", shuffle=False):
        """Generate a mosaic based on artworks list"""
        ARTWORK_SIZE = 640
        if shuffle is True:
            random.shuffle(artworks)
        max_tiles = math.floor(math.sqrt(len(artworks)))
        tiles = min(size, max_tiles)
        artworks = artworks[:tiles**2]
        images = [Image.open(requests.get(artwork, stream=True).raw) for artwork in artworks]
        new_image = Image.new('RGB', (tiles * ARTWORK_SIZE, tiles * ARTWORK_SIZE))
        current_image = 0
        for x in range(tiles):
            for y in range(tiles):
                new_image.paste(images[current_image], (x * ARTWORK_SIZE, y * ARTWORK_SIZE))
                current_image += 1
        if output is None:
            return new_image
        new_image.save(output)
        return output

    def create(self, playlist, size=2, output="mosaic.png", shuffle=False):
        """Generate a mosaic from playlist"""
        albums = self.get_albums(playlist)
        artworks = self.get_artworks(albums)
        return self.generate_mosaic(artworks, size=size, output=output, shuffle=shuffle)

    @staticmethod
    def uri_to_api(uri):
        """Generate a dict containing playlist based on Spotify Playlist URI"""
        m = re.match('^spotify:user:(.+?):playlist:(.+?)$', uri)
        return "https://api.spotify.com/v1/users/{}/playlists/{}/tracks".format(m.group(1), m.group(2))

    @property
    def token(self):
        """Get token Bearer"""
        if self._token is not None:
            return self._token

        url = "https://accounts.spotify.com/api/token"
        headers = {
            "Authorization": "Basic " + base64.b64encode(("{}:{}".format(self.spotifyid, self.spotifysecret)).encode("utf-8")).decode("utf-8")
        }
        data = {
            "grant_type": "client_credentials"
        }
        r = requests.post(url, headers=headers, data=data)

        self._token = "{} {}".format(r.json()["token_type"], r.json()["access_token"])
        return self._token

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a mosaic from spotify playlist.")

    parser.add_argument("spotifyid", help="Spotify API Client ID")
    parser.add_argument("spotifysecret", help="Spotify API Client Secret")
    parser.add_argument("playlist", help="Spotify playlist URI")

    parser.add_argument("-t", "--tiles", help="number of artworks per mosaic row",
                        type=int, default=2)
    parser.add_argument("-o", "--out", help="output file",
                        default="mosaic.png")
    parser.add_argument("-s", "--shuffle", help="randomize the order of artworks",
                        action="store_true")
    parser.add_argument("-r", "--resolution", help="select the resolution of one artwork",
                        type=int, default=640, choices=[64, 300, 640])

    args = parser.parse_args()

    mosaic = SpotifyMosaic({
        "id": args.spotifyid,
        "secret": args.spotifysecret
    })

    mosaic.create(args.playlist, size=args.tiles, output=args.out, shuffle=args.shuffle)
