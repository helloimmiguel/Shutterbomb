use image::{ImageReader, ImageBuffer, RgbaImage, DynamicImage};
use rand::{rng, Rng};

pub fn main(input_path: &str, output_path: &str, iso: &i32) {
    let img = ImageReader::open(input_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let mut rawimg = img.to_rgba8().into_raw();
    let intensity = 0.01 * *iso as f32 / 1000.0; // More reasonable intensity calculation
    let mut rng = rng();

    for byte in rawimg.iter_mut() {
        if rng.random_bool(intensity as f64) {
            *byte = rng.random_range(0..=255);
        }
    }

    let new_img: RgbaImage = ImageBuffer::from_raw(img.width(), img.height(), rawimg)
        .expect("Failed to create new image");

    let rgb_image = DynamicImage::ImageRgba8(new_img).to_rgb8();
    rgb_image.save(output_path).expect("Failed to save image");
}
