use crate::args::CliArgs;
use crate::auth::auth_with_credentials;
use futures::{pin_mut, TryStreamExt};
use rspotify::{
    clients::BaseClient,
    model::{Id, PlaylistId, PlaylistItem},
};

pub async fn run(args: CliArgs) -> Result<(), &'static str> {
    let client = auth_with_credentials(&args.credentials).await?;

    generate_mosaic(&client, &args.playlist_uri).await?;

    Ok(())
}

async fn get_song_list(
    client: &impl BaseClient,
    playlist_id: &PlaylistId,
) -> Result<Vec<PlaylistItem>, &'static str> {
    let mut vec = Vec::new();

    let stream = client.playlist_items(playlist_id, None, None);
    pin_mut!(stream);
    while let Some(item) = stream
        .try_next()
        .await
        .or(Err("Could not fetch some of playlist's songs!"))?
    {
        vec.push(item);
    }

    Ok(vec)
}

pub async fn generate_mosaic(
    client: &impl BaseClient,
    playlist_uri: &str,
) -> Result<(), &'static str> {
    let playlist_id = PlaylistId::from_uri(playlist_uri).or(Err("Invalid playlist URI!"))?;

    dbg!(get_song_list(client, &playlist_id).await?);

    Ok(())
}
