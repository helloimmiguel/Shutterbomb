use image::{DynamicImage, ImageBuffer, ImageReader, RgbImage};
use rand::{Rng, rng};

pub struct SynesthesiaState {
    pub rawimg: Vec<u8>,
    pub rng: rand::rngs::ThreadRng,
    pub modifications_count: usize,
    pub img_width: u32,
    pub img_height: u32,
}

impl SynesthesiaState {
    pub fn new(input_path: &str) -> Result<Self, String> {
        let img = ImageReader::open(input_path)
            .map_err(|e| format!("Failed to open image: {}", e))?;
        
        let img = img.decode()
            .map_err(|e| format!("Failed to decode image: {}", e))?;

        let rawimg = img.to_rgba8().into_raw();
        
        Ok(Self {
            rawimg,
            rng: rng(),
            modifications_count: 0,
            img_width: img.width(),
            img_height: img.height(),
        })
    }

    pub fn process_key(&mut self, c: char) -> String {
        let value = c as u8;
        let chaos_amount = (value as usize * 13) % 500 + 50;
        
        for _ in 0..chaos_amount {
            let random_index = self.rng.random_range(0..self.rawimg.len());
            
            match c {
                'a'..='z' => {
                    self.rawimg[random_index] = self.rawimg[random_index].wrapping_add(value);
                }
                '0'..='9' => {
                    self.rawimg[random_index] = value.wrapping_mul(17);
                }
                ' ' => {
                    self.rawimg[random_index] = 0;
                }
                _ => {
                    self.rawimg[random_index] = self.rng.random_range(0..=255);
                }
            }
        }
        
        self.modifications_count += chaos_amount;
        format!("🎵 Key '{}' pressed - {} pixels databent! (Total: {})", c, chaos_amount, self.modifications_count)
    }

    pub fn save(&self, output_path: &str) -> Result<String, String> {
        let rgb_data: Vec<u8> = self.rawimg
            .chunks(4)
            .flat_map(|rgba| &rgba[..3])
            .copied()
            .collect();

        let new_img: RgbImage = ImageBuffer::from_raw(self.img_width, self.img_height, rgb_data)
            .ok_or("Failed to create new image")?;
        
        DynamicImage::ImageRgb8(new_img)
            .save(output_path)
            .map_err(|e| format!("Failed to save image: {}", e))?;

        Ok(format!("🎭 Synesthesia complete! {} pixels modified total", self.modifications_count))
    }
}

