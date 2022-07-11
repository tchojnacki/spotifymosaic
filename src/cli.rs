use crate::api::get_cover_urls;
use crate::args::CliArgs;
use crate::auth::auth_with_client_creds;
use crate::images::{blur_mosaic, generate_mosaic};

pub async fn run(args: CliArgs) -> Result<(), &'static str> {
    let client = auth_with_client_creds(&args.credentials).await?;

    let cover_urls = get_cover_urls(
        &client,
        &args.playlist_uri,
        args.tile_side_len,
        args.arrangement,
    )
    .await?;

    let tile_count = cover_urls.len() as u32;

    let image = generate_mosaic(cover_urls, args.resolution).await?;
    let image = blur_mosaic(image, args.blur as f32 / 100.0, tile_count);

    image
        .save(args.output_path)
        .or(Err("Could not save the image!"))?;

    Ok(())
}
