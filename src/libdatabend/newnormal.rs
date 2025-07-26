use image::{DynamicImage, ImageBuffer, ImageReader, RgbImage};
use rand::Rng;
use rand::rng;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub fn main(input_path: &str, output_path: &str) {
    let mut img = ImageReader::open(input_path)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8()
        .into_raw();

    enable_raw_mode().expect("failed to enable raw mode");

    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(_) => {
                        for _ in 0..img.len() / 16 { 
                            let mut luck = rng();
                            let idx = luck.random_range(0..img.len());
                            img[idx] = luck.random_range(0..=255);
                        }
                    }
                    KeyCode::Esc => {
                        println!("It's time to step out onto the new normal...");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode().expect("failed to disable raw mode");

    let (width, height) = {
        let original_img = ImageReader::open(input_path)
            .unwrap()
            .decode()
            .unwrap();
        (original_img.width(), original_img.height())
    };

    let rgba_img: RgbImage = ImageBuffer::from_raw(width, height, img)
        .expect("failed to create image buffer");

    let dynamic_img = DynamicImage::ImageRgb8(rgba_img);
    dynamic_img.save(output_path).expect("failed to save image");
}