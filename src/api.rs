use crate::args::TileArrangement;
use futures::{pin_mut, TryStreamExt};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rspotify::{
    clients::BaseClient,
    model::{Id, PlayableItem, PlaylistId, SimplifiedAlbum},
};
use std::collections::HashSet;

async fn get_playlist_unique_albums(
    client: &impl BaseClient,
    playlist_id: PlaylistId<'_>,
) -> Result<Vec<SimplifiedAlbum>, &'static str> {
    let mut albums = Vec::new();
    let mut used_ids = HashSet::new();

    let stream = client.playlist_items(playlist_id, None, None);
    pin_mut!(stream);
    while let Some(item) = stream
        .try_next()
        .await
        .or(Err("Could not fetch some of playlist's songs!"))?
    {
        if let Some(PlayableItem::Track(track)) = item.track {
            if let Some(album_id) = &track.album.id {
                if used_ids.insert(album_id.id().to_owned()) {
                    albums.push(track.album);
                }
            }
        }
    }

    Ok(albums)
}

#[must_use]
fn arrange_albums(
    mut albums: Vec<SimplifiedAlbum>,
    total_tile_count: usize,
    arrangement: TileArrangement,
) -> Vec<SimplifiedAlbum> {
    match arrangement {
        TileArrangement::First => {}
        TileArrangement::Last => {
            let rotation = albums.len() - total_tile_count;
            albums.rotate_left(rotation);
        }
        TileArrangement::Random => {
            albums.shuffle(&mut thread_rng());
        }
    };

    albums.truncate(total_tile_count);

    albums
}

#[must_use]
fn select_cover_urls(albums: &[SimplifiedAlbum]) -> Vec<String> {
    albums
        .iter()
        .map(|album| {
            album
                .images
                .iter()
                .max_by_key(|img| img.width.unwrap_or(0).min(img.height.unwrap_or(0)))
                .unwrap()
                .url
                .to_owned()
        })
        .collect()
}

pub async fn get_cover_urls(
    client: &impl BaseClient,
    playlist_uri: &str,
    tile_side_len: u32,
    arrangement: TileArrangement,
) -> Result<Vec<String>, &'static str> {
    let playlist_id = PlaylistId::from_uri(playlist_uri).or(Err("Invalid playlist URI!"))?;
    let albums = get_playlist_unique_albums(client, playlist_id).await?;
    let tile_side_len = tile_side_len.min((albums.len() as f64).sqrt() as u32);
    let albums = arrange_albums(albums, tile_side_len.pow(2) as usize, arrangement);
    let urls = select_cover_urls(&albums);

    Ok(urls)
}
