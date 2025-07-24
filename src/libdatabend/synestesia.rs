use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use rand::{rng, Rng};
use image::{ImageBuffer, ImageReader, RgbaImage, DynamicImage};

pub fn main(input_path: &str, output_path: &str) {
    let img = ImageReader::open(input_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let mut rawimg = img.to_rgba8().into_raw();
    let mut rng = rng();

    enable_raw_mode().expect("failed to enable raw mode");

    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        let value = c as u8;
                        let random_index = rng.random_range(0..rawimg.len());
                        rawimg[random_index] = value;
                        println!("Applied '{}' ({} as u8) at {}", c, value, random_index);
                    }
                    KeyCode::Esc => {
                        println!("Exiting synesthesia mode...");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode().expect("failed to disable raw mode");

    let new_img: RgbaImage =
        ImageBuffer::from_raw(img.width(), img.height(), rawimg).expect("Failed to create new image");
    DynamicImage::ImageRgba8(new_img).save(output_path).expect("Failed to save new image");
}
