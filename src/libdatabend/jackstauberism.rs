use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use image::{ImageBuffer, ImageReader, RgbImage, DynamicImage};
use rand::{rng, Rng};
use std::time::Duration;
use std::error::Error;

pub fn main(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let original_img = ImageReader::open(input_path)?
        .decode()?
        .to_rgba8();
    let (width, height) = original_img.dimensions();
    let mut img = original_img.into_raw();

    enable_raw_mode()?;

    let lyrics = r#"
    Goodnight, little eye
    The moon, the sun descending
    Can I run a lie?
    Imagine a life as it's ending
    Goodnight, sweetheart
    Thicker thighs for my friends
    Too young, too hard 
    Imagine a life as it's ending
    I begin to databend
    Nothing in, prize out
    I begin to Databend
    Not to win, prize out
    Don't show me why
    The future life's depending
    On your better mind
    Cut up and condescending
    Goodnight, my love
    Thicker thighs for my friends
    My hopes are above
    Where all of your data's landing
    I begin to databend
    Nothing in, prize out
    I begin to databend
    Nothing in, prize out
    Not to win, not to win, not to win
    Nothing in, nothing in, nothing in (prize out)
    Not to win, not to win, not to win
    Nothing in, nothing in, nothing in (prize out)
    I begin to databend
    Nothing in, prize out
    I begin to databend
    Nothing in, prize out
    Uh, uh, gotta ask you
    Girl, get up, I gotta ask you
    Gotta, gotta, gotta ask you 
    Girl, get up, I gotta ask you
    Gotta, gotta, ask you
    Gotta, gotta, ask you
    Gotta, gotta
    Gotta, gotta
    "#;
    let mut lyric_index = 0;
    let mut rng = rng();
    let lyrics_bytes = lyrics.as_bytes();

    loop {
        if event::poll(Duration::from_millis(500))?
            && let Ok(Event::Key(key_event)) = event::read()
        {
            match key_event.code {
                KeyCode::Char(_) => {
                    for _ in 0..img.len() / 128 {
                        let idx = rng.random_range(0..img.len());
                        img[idx] = lyrics_bytes[lyric_index];
                        lyric_index = (lyric_index + 1) % lyrics_bytes.len();
                    }
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;

    let new_img: RgbImage = ImageBuffer::from_raw(width, height, img)
        .expect("Failed to create new image");
    DynamicImage::ImageRgb8(new_img).save(output_path)?;

    Ok(())
}
