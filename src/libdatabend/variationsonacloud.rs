use image::{DynamicImage, ImageBuffer, ImageReader, RgbImage};
use rand::{Rng, rng};

pub fn main(input_path: &str, output_path: &str, patch_size: u32) -> Result<(), String> {
    let img = ImageReader::open(input_path)
        .map_err(|e| format!("Failed to open image: {}", e))?;
    
    let img = img.decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let mut rng = rng();
    let rawimg = img.to_rgba8().into_raw();
    let height = img.height();
    let width = img.width();

    if patch_size > width || patch_size > height {
        return Err("Patch size must be smaller than image dimensions".to_string());
    }

    let mut new_rawimg = rawimg.clone();
    
    // Calculate how many patches fit
    let patches_horizontal = width / patch_size;
    let patches_vertical = height / patch_size;
    
    // Create a list of all patch positions
    let mut patch_positions: Vec<(u32, u32)> = Vec::new();
    for y in 0..patches_vertical {
        for x in 0..patches_horizontal {
            patch_positions.push((x * patch_size, y * patch_size));
        }
    }
    
    // Shuffle the positions
    for i in 0..patch_positions.len() {
        let j = rng.random_range(0..patch_positions.len());
        patch_positions.swap(i, j);
    }
    
    // Copy patches to their new shuffled positions
    let mut dest_index = 0;
    for grid_y in 0..patches_vertical {
        for grid_x in 0..patches_horizontal {
            let src_x = grid_x * patch_size;
            let src_y = grid_y * patch_size;
            let (dest_x, dest_y) = patch_positions[dest_index];
            dest_index += 1;
            
            // Copy the patch from source position to destination position
            for y in 0..patch_size {
                for x in 0..patch_size {
                    let src_pixel_index = ((src_y + y) * width + (src_x + x)) * 4;
                    let dest_pixel_index = ((dest_y + y) * width + (dest_x + x)) * 4;
                    
                    let src_start = src_pixel_index as usize;
                    let dest_start = dest_pixel_index as usize;
                    
                    if src_start + 4 <= rawimg.len() && dest_start + 4 <= new_rawimg.len() {
                        for i in 0..4 {
                            new_rawimg[dest_start + i] = rawimg[src_start + i];
                        }
                    }
                }
            }
        }
    }

    let rgb_data: Vec<u8> = new_rawimg
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
