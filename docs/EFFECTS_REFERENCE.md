# 🎨 Effects Reference Documentation

Comprehensive documentation for all databending effects in Shutterbomb.

## 📋 Table of Contents

1. [Effect Overview](#effect-overview)
2. [Parametric Effects](#parametric-effects)
3. [Interactive Effects](#interactive-effects)
4. [Technical Implementation](#technical-implementation)
5. [Troubleshooting](#troubleshooting)

## Effect Overview

### Classification

**Parametric Effects** (require configuration):
- 📸 Oversensibility
- ☀️ Overexposure
- ☁️ Variations on a Cloud
- ⚡ The Mind Electric

**Interactive Effects** (real-time user input):
- 🎹 Synesthesia
- 🎵 Jack Stauberism
- 🌈 New Normal

## Parametric Effects

### 📸 Oversensibility (`oversensibility.rs`)

**Purpose**: Simulates camera sensor noise at high ISO settings.

**Parameters**:
- `ISO`: Integer (0-6400)
  - 0-800: Minimal noise
  - 800-1600: Light grain
  - 1600-3200: Moderate corruption
  - 3200-9999: Heavy noise

**Algorithm**:
```rust
let intensity = 0.01 * iso as f32 / 1000.0;
for byte in rawimg.iter_mut() {
    if rng.random_bool(intensity as f64) {
        *byte = rng.random_range(0..=255);
    }
}
```

**Technical Details**:
- **Probability-based**: Each byte has `intensity` chance of corruption
- **Random Replacement**: Corrupted bytes become completely random values
- **Channel Agnostic**: Affects R, G, B, and A channels equally

**Best Practices**:
- Start with ISO 800 for subtle effects
- ISO 3200+ for artistic corruption
- Consider image content - busy images hide noise better

---

### ☀️ Overexposure (`overexposure.rs`)

**Purpose**: Creates blown-out highlights and exposure artifacts.

**Parameters**:
- `Exposure Factor`: Float (0.1-3.0)
  - 0.1-0.5: Subtle brightening
  - 0.5-1.5: Realistic overexposure
  - 1.5-3.0: Extreme highlights

**Algorithm**:
```rust
let boost_max = (50.0 * exposure_factor).max(1.0) as u8;
for chunk in rawimg.chunks_mut(4) {
    for channel in chunk.iter_mut().take(3) { // Skip alpha channel
        let boost: u8 = rng.random_range(0..boost_max);
        let sum = *channel as u16 + boost as u16;
        *channel = sum.min(255) as u8;
    }
}
```

**Technical Details**:
- **Pixel-wise Processing**: Works on RGBA chunks (4 bytes per pixel)
- **RGB Only**: Preserves alpha channel integrity
- **Clamping**: Prevents overflow beyond 255
- **Random Boost**: Each RGB channel gets random brightness increase

**Visual Effects**:
- Creates realistic camera overexposure artifacts
- Produces white/bright spots randomly distributed
- Maintains image structure while adding highlights

---

### ☁️ Variations on a Cloud (`variationsonacloud.rs`)

**Purpose**: Creates glitch patches by copying random image regions.

**Parameters**:
- `Patch Size`: Integer (10-200)
  - 10-50: Small glitch spots
  - 50-100: Medium geometric artifacts
  - 100-200: Large region swaps

**Algorithm**:
```rust
let sx = rng.gen_range(0..(width - patch_size));
let sy = rng.gen_range(0..(height - patch_size));
let tx = rng.gen_range(0..(width - patch_size));
let ty = rng.gen_range(0..(height - patch_size));

for y in 0..patch_size {
    for x in 0..patch_size {
        let src_index = ((sy + y) * width + (sx + x)) * 4;
        let dest_index = ((ty + y) * width + (tx + x)) * 4;
        // Copy 4 bytes (RGBA pixel)
        new_rawimg[dest_index..dest_index + 4]
            .copy_from_slice(&rawimg[src_index..src_index + 4]);
    }
}
```

**Technical Details**:
- **Rectangle Copying**: Copies square patches between random locations
- **RGBA Preservation**: Maintains full pixel data (including alpha)
- **Boundary Checking**: Ensures patches fit within image dimensions
- **Single Operation**: One patch copy per execution

---

### ⚡ The Mind Electric (`themindelectric.rs`)

**Purpose**: Creates complex layered effects with alpha blending and color shifts.

**Parameters**:
- `Layers`: Integer (1-20)
  - 1-5: Subtle color bleeding
  - 5-10: Complex overlays
  - 10-20: Dense, chaotic results

**Algorithm**:
```rust
for _ in 0..*layers {
    let offset_x = rng.random_range(0..width);
    let offset_y = rng.random_range(0..height);
    let alpha_mult = rng.random_range(0.1..0.3);

    let color_shift = (
        rng.random_range(0..256),
        rng.random_range(0..256),
        rng.random_range(0..256),
    );

    // Process each pixel with offset and color shift
    // Apply alpha blending to canvas
}
```

**Alpha Blending Implementation**:
```rust
fn alpha_blend(bottom: Rgba<u8>, top: Rgba<u8>) -> Rgba<u8> {
    let alpha_top = top[3] as f32 / 255.0;
    let alpha_bottom = bottom[3] as f32 / 255.0;
    let out_alpha = alpha_top + alpha_bottom * (1.0 - alpha_top);
    
    // Standard over-operator alpha compositing
}
```

**Technical Details**:
- **Layer Composition**: Each layer applies random offset and color shift
- **Proper Alpha Math**: Implements correct alpha compositing formulas
- **Color Shifting**: Random RGB offsets for each layer
- **Transparent Canvas**: Starts with transparent black background

**Performance Characteristics**:
- **Quadratic Complexity**: O(layers × width × height)
- **Memory Intensive**: Creates full canvas for composition
- **CPU Heavy**: Most computationally expensive effect

## Interactive Effects

### 🎹 Synesthesia (`synestesia.rs`)

**Purpose**: Real-time databending through keyboard input.

**Interaction Model**:
- Any character key: Injects ASCII value at random position
- Esc: Exit and save result

**Algorithm**:
```rust
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
```

**Technical Details**:
- **Raw Terminal Mode**: Direct key capture without buffering
- **ASCII Mapping**: Character codes become pixel data
- **Random Injection**: Each keypress affects random byte position
- **Immediate Feedback**: Changes accumulate in real-time

**Creative Usage**:
- Type words to embed text in image data
- Use special characters for unique byte patterns
- Rhythmic typing creates interesting distributions
- Long sessions build up complex corruption

---

### 🎵 Jack Stauberism (`jackstauberism.rs`)

**Purpose**: Embeds song lyrics into image data for textual corruption.

**Embedded Content**:
```rust
let lyrics = r#"
Goodnight, little eye
The moon, the sun descending
Can I run a lie?
Imagine a life as it's ending
...
I begin to databend
Nothing in, prize out
...
"#;
```

**Interaction Model**:
- Any key: Injects lyrics characters sequentially
- Automatic progression through lyrics
- Esc: Exit and save

**Algorithm**:
```rust
for _ in 0..img.len() / 128 {  // Process 1/128th of image per keypress
    let idx = rng.random_range(0..img.len());
    img[idx] = lyrics_bytes[lyric_index];
    lyric_index = (lyric_index + 1) % lyrics_bytes.len();
}
```

**Technical Details**:
- **Sequential Injection**: Lyrics progress character by character
- **Batch Processing**: Multiple injections per keypress
- **Cyclic Buffer**: Lyrics wrap around when exhausted
- **Artistic Concept**: Merges textual and visual art

---

### 🌈 New Normal (`newnormal.rs`)

**Purpose**: Aggressive random corruption for abstract results.

**Interaction Model**:
- Any character key: Triggers corruption wave
- Multiple keypresses: Accumulative damage
- Esc: Exit with message "It's time to step out onto the new normal..."

**Algorithm**:
```rust
KeyCode::Char(_) => {
    for _ in 0..img.len() / 16 {  // Corrupt 1/16th of image
        let idx = luck.random_range(0..img.len());
        img[idx] = luck.random_range(0..=255);
    }
}
```

**Technical Details**:
- **Mass Corruption**: Each keypress affects 6.25% of image data
- **Uniform Random**: Completely random byte replacement
- **Accumulative**: Multiple keypresses stack corruption
- **Character Agnostic**: All non-Esc keys have same effect

**Creative Philosophy**:
- "New Normal" refers to accepting chaos as the default state
- Progressive destruction through repeated interaction
- Abstract art through data annihilation

## Technical Implementation

### Common Patterns

#### Image Loading Standard
```rust
let img = ImageReader::open(input_path)
    .expect("Failed to open image")
    .decode()
    .expect("Failed to decode image");
let mut rawimg = img.to_rgba8().into_raw();
```

#### Interactive Mode Standard
```rust
enable_raw_mode().expect("failed to enable raw mode");
// ... processing loop ...
disable_raw_mode().expect("failed to disable raw mode");
```

#### Image Saving Standard
```rust
let new_img: RgbaImage = ImageBuffer::from_raw(width, height, rawimg)
    .expect("Failed to create new image");
DynamicImage::ImageRgba8(new_img).save(output_path)
    .expect("Failed to save image");
```

### Error Handling Patterns

1. **Fatal Errors**: Use `.expect()` with descriptive messages for unrecoverable errors
2. **Parameter Defaults**: Use `.unwrap_or()` with sensible defaults for user-provided values
3. **Recoverable Errors**: Return `Result` types for operations that may fail gracefully

### Performance Characteristics

| Effect | Complexity | Memory Usage | Processing Time |
|--------|------------|--------------|-----------------|
| Oversensibility | O(n) | Low | Fast |
| Overexposure | O(n) | Low | Fast |
| Variations on a Cloud | O(patch²) | Medium | Fast |
| The Mind Electric | O(layers × w × h) | High | Slow |
| Synesthesia | O(keystrokes) | Low | Interactive |
| Jack Stauberism | O(keystrokes) | Low | Interactive |
| New Normal | O(keystrokes) | Low | Interactive |

## Troubleshooting

### Runtime Issues

1. **Terminal State**: Always ensure `disable_raw_mode()` is called on exit
2. **File Paths**: Use absolute paths for testing
3. **Memory**: Monitor usage with large images
4. **Interactive Feedback**: Check the status bar for mode-specific help

---

*Complete reference for all databending effects*