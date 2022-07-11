use crate::api::get_cover_urls;
use crate::args::CliArgs;
use crate::auth::auth_with_client_creds;
use crate::images::generate_mosaic;

pub async fn run(args: CliArgs) -> Result<(), &'static str> {
    let client = auth_with_client_creds(&args.credentials).await?;

    let cover_urls = get_cover_urls(
        &client,
        &args.playlist_uri,
        args.tile_side_len,
        args.arrangement,
    )
    .await?;

    let image = generate_mosaic(cover_urls, args.resolution).await?;

    image
        .save(args.output_path)
        .or(Err("Could not save the image!"))?;

    Ok(())
}
