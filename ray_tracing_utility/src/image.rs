use image::io::Reader;
use image::{ColorType, DynamicImage, ImageFormat};
use std::error::Error;

pub fn save_image(filename: &str, cx: usize, cy: usize, pixel_data: &Vec<u8>) {
    image::save_buffer_with_format(
        filename,
        &pixel_data,
        cx as u32,
        cy as u32,
        ColorType::Rgba8,
        ImageFormat::Png,
    )
    .unwrap();
}

pub fn load_image(filename: &str) -> Result<(usize, usize, Vec<u8>), Box<dyn Error>> {
    let rgba_image = Reader::open(filename)?.decode()?.to_rgba8();
    Ok((
        rgba_image.width() as usize,
        rgba_image.height() as usize,
        DynamicImage::ImageRgba8(rgba_image).into_bytes(),
    ))
}
