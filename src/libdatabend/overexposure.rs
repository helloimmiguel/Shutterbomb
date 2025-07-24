use image::{DynamicImage, ImageBuffer, ImageReader, RgbaImage};
use rand::{rng, Rng};

pub fn main(input_path: &str, output_path: &str, exposure_factor: f32) {
    let img = ImageReader::open(input_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let mut rawimg = img.to_rgba8().into_raw();
    let mut rng = rng();

    for chunk in rawimg.chunks_mut(4) {
        for i in 0..3 {
            let boost: u8 = rng.random_range(0..(50.0 * exposure_factor) as u8);
            let sum = chunk[i] as u16 + boost as u16;
            chunk[i] = sum.min(255) as u8;
        }
    }
    let new_img: RgbaImage = ImageBuffer::from_raw(img.width(), img.height(), rawimg)
        .expect("Failed to create new image");

    let rgb_image = DynamicImage::ImageRgba8(new_img).to_rgb8();
    rgb_image.save(output_path).expect("Failed to save image");
}
