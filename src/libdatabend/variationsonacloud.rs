use image::{ImageBuffer, ImageReader, RgbaImage, DynamicImage};
use rand::{thread_rng, Rng};

pub fn main(input_path: &str, output_path: &str, patch_size: u32) {
    let img = ImageReader::open(input_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let mut rng = thread_rng();
    let rawimg = img.to_rgba8().into_raw();
    let height = img.height();
    let width = img.width();
    
    if patch_size > width || patch_size > height {
        panic!("Patch size must be smaller than image dimensions");
    }

    let sx = rng.gen_range(0..(width - patch_size));
    let sy = rng.gen_range(0..(height - patch_size));
    let tx = rng.gen_range(0..(width - patch_size));
    let ty = rng.gen_range(0..(height - patch_size));

    let mut new_rawimg = rawimg.clone();
    for y in 0..patch_size {
        for x in 0..patch_size {
            let src_index = ((sy + y) * width + (sx + x)) * 4;
            let dest_index = ((ty + y) * width + (tx + x)) * 4;
            let src_start = src_index as usize;
            let src_end = src_start + 4;
            let dest_start = dest_index as usize;
            let dest_end = dest_start + 4;
            
            if src_end <= rawimg.len() && dest_end <= new_rawimg.len() {
                new_rawimg[dest_start..dest_end].copy_from_slice(&rawimg[src_start..src_end]);
            }
        }
    }

    let new_img: RgbaImage = ImageBuffer::from_raw(width, height, new_rawimg)
        .expect("Failed to create new image");
    DynamicImage::ImageRgba8(new_img).save(output_path).expect("Failed to save image");
}
