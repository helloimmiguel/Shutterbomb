use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba, RgbImage};
use rand::{Rng, rng};
use std::error::Error;

pub fn main(input_path: &str, output_path: &str, layers: &u32) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(input_path)?.decode()?;
    let mut rng = rng();
    let (width, height) = (img.width(), img.height());
    let mut canvas = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 0]));

    for _ in 0..*layers {
        let offset_x = rng.random_range(0..width);
        let offset_y = rng.random_range(0..height);
        let alpha_mult = rng.random_range(0.1..0.3);

        let color_shift = (
            rng.random_range(0..256),
            rng.random_range(0..256),
            rng.random_range(0..256),
        );

        for y in 0..height {
            for x in 0..width {
                let src_x = x as i32 - offset_x as i32;
                let src_y = y as i32 - offset_y as i32;

                if src_x >= 0 && src_x < width as i32 && src_y >= 0 && src_y < height as i32 {
                    let pixel = img.get_pixel(src_x as u32, src_y as u32);

                    let r = (pixel[0] as i32 + color_shift.0).clamp(0, 255) as u8;
                    let g = (pixel[1] as i32 + color_shift.1).clamp(0, 255) as u8;
                    let b = (pixel[2] as i32 + color_shift.2).clamp(0, 255) as u8;
                    let a = (pixel[3] as f32 * alpha_mult) as u8;

                    let new_pixel = Rgba([r, g, b, a]);

                    let dst_pixel = canvas.get_pixel_mut(x, y);
                    *dst_pixel = alpha_blend(*dst_pixel, new_pixel);
                }
            }
        }
    }

    let rgb_data: Vec<u8> = canvas
        .chunks(4)
        .flat_map(|rgba| &rgba[..3])
        .copied()
        .collect();

    let new_img: RgbImage = ImageBuffer::from_raw(width, height, rgb_data)
        .ok_or("Failed to create new image")?;

    DynamicImage::ImageRgb8(new_img)
        .save(output_path)
        .map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(())
}

fn alpha_blend(bottom: Rgba<u8>, top: Rgba<u8>) -> Rgba<u8> {
    let alpha_top = top[3] as f32 / 255.0;
    let alpha_bottom = bottom[3] as f32 / 255.0;
    let out_alpha = alpha_top + alpha_bottom * (1.0 - alpha_top);

    if out_alpha == 0.0 {
        return Rgba([0, 0, 0, 0]);
    }

    let r = ((top[0] as f32 * alpha_top + bottom[0] as f32 * alpha_bottom * (1.0 - alpha_top))
        / out_alpha)
        .round() as u8;
    let g = ((top[1] as f32 * alpha_top + bottom[1] as f32 * alpha_bottom * (1.0 - alpha_top))
        / out_alpha)
        .round() as u8;
    let b = ((top[2] as f32 * alpha_top + bottom[2] as f32 * alpha_bottom * (1.0 - alpha_top))
        / out_alpha)
        .round() as u8;
    let a = (out_alpha * 255.0).round() as u8;

    Rgba([r, g, b, a])
}
