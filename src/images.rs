use image::imageops;
use image::RgbImage;

pub async fn generate_mosaic(urls: Vec<String>, resolution: u32) -> Result<RgbImage, &'static str> {
    let tile_side_len = (urls.len() as f64).sqrt() as u32;
    let tile_resolution = resolution / tile_side_len;

    let mut image = RgbImage::new(resolution, resolution);

    for (index, url) in urls.iter().enumerate() {
        let index = index as u32;
        let (x, y) = (index % tile_side_len, index / tile_side_len);

        let cover = image::load_from_memory(
            &reqwest::get(url)
                .await
                .or(Err("Could not download one of the covers!"))?
                .bytes()
                .await
                .or(Err("Could not convert one of the covers to bytes!"))?,
        )
        .or(Err("Could not parse one of the covers!"))?
        .resize(
            tile_resolution,
            tile_resolution,
            imageops::FilterType::Triangle,
        )
        .into_rgb8();

        imageops::overlay(
            &mut image,
            &cover,
            (x * tile_resolution).into(),
            (y * tile_resolution).into(),
        );
    }

    let resolution = tile_resolution * tile_side_len;
    let image = imageops::crop_imm(&image, 0, 0, resolution, resolution).to_image();

    Ok(image)
}

pub fn blur_mosaic(image: RgbImage, coefficient: f32, tile_count: u32) -> RgbImage {
    let tile_side_len = (tile_count as f32).sqrt();
    let img_resolution = image.dimensions().0 as f32;

    if coefficient.abs() < f32::EPSILON {
        image
    } else {
        imageops::blur(&image, (img_resolution / tile_side_len) * 0.5 * coefficient)
    }
}
