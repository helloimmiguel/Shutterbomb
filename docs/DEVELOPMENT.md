# 🛠️ Development Guide

Guide for developers working on or extending Shutterbomb.

## 🏗️ Architecture Overview

### Main Components

1. **TUI Layer** (`main.rs`)
   - Terminal interface using `ratatui`
   - Event handling with `crossterm`
   - Application state management

2. **Effect Library** (`libdatabend/`)
   - Modular effect implementations
   - Consistent interface across effects
   - Image processing utilities

### Data Flow

```
User Input → TUI Event Handler → App State → Effect Execution → Image Processing → File Output
```

## 🔧 Adding New Effects

### 1. Create Effect Module

Create a new file in `src/libdatabend/`:

```rust
// src/libdatabend/my_effect.rs
use image::{ImageReader, ImageBuffer, RgbaImage, DynamicImage};

pub fn main(input_path: &str, output_path: &str, param: SomeType) {
    // Load image
    let img = ImageReader::open(input_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    
    // Process image
    let mut rawimg = img.to_rgba8().into_raw();
    
    // Your effect logic here
    
    // Save result
    let new_img: RgbaImage = ImageBuffer::from_raw(img.width(), img.height(), rawimg)
        .expect("Failed to create new image");
    DynamicImage::ImageRgba8(new_img).save(output_path)
        .expect("Failed to save image");
}
```

### 2. Register in Module

Add to `src/libdatabend/mod.rs`:
```rust
pub mod my_effect;
```

### 3. Add to Effects List

In `main.rs`, add to the `effects` vector in `App::new()`:
```rust
DatabendEffect {
    name: "My Effect".to_string(),
    description: "Description of what it does".to_string(),
    emoji: "🎯".to_string(),
    params: vec!["Parameter Name".to_string()],
},
```

### 4. Add to Execution Logic

In the `execute_effect()` method match statement:
```rust
7 => {
    let param = self.params[0].parse::<SomeType>().unwrap_or(default_value);
    libdatabend::my_effect::main(&self.input_path, &self.output_path, param);
}
```

## 🎨 Effect Design Patterns

### Simple Parameter Effects
```rust
pub fn main(input_path: &str, output_path: &str, intensity: f32) {
    // Load, process with intensity, save
}
```

### Interactive Effects
```rust
pub fn main(input_path: &str, output_path: &str) {
    enable_raw_mode()?;
    
    loop {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Esc => break,
                        // Handle other keys
                    }
                }
            }
        }
    }
    
    disable_raw_mode()?;
}
```

### Multi-Parameter Effects
```rust
pub fn main(input_path: &str, output_path: &str, params: &[String]) {
    let param1 = params[0].parse().unwrap_or(default);
    let param2 = params[1].parse().unwrap_or(default);
    // Process with multiple parameters
}
```

## 🔧 Code Style Guidelines

### Error Handling
- Use `expect()` with descriptive messages for unrecoverable errors
- Use `unwrap_or()` for parameter parsing with sensible defaults
- Return `Result` types for complex operations

### Naming Conventions
- Effect files: lowercase with underscores (`my_effect.rs`)
- Function names: descriptive and consistent (`main`)
- Parameters: clear, type-appropriate names

### Performance Considerations
- Clone image data at start of processing
- Prefer in-place modifications when possible
- Use iterators for pixel processing
- Consider memory usage for large images

## 🧪 Testing

### Manual Testing Workflow
1. Create test images of various sizes
2. Test with extreme parameter values
3. Verify output image integrity
4. Test interactive effects thoroughly

### Adding Automated Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_effect_basic() {
        // Test basic functionality
    }
}
```

## 📚 Dependencies Management

### Current Dependencies
- `ratatui`: TUI framework
- `crossterm`: Cross-platform terminal
- `image`: Image processing
- `rand`: Random number generation

### Adding New Dependencies
1. Add to `Cargo.toml`
2. Update documentation
3. Consider license compatibility
4. Test cross-platform compatibility

## 🐛 Debugging

### Common Issues
- **Terminal state corruption**: Ensure `disable_raw_mode()` is called
- **Image format errors**: Check input file validity
- **Memory issues**: Monitor with large images
- **Path problems**: Use absolute paths for testing

### Debug Techniques
- Add `println!` statements for interactive effects
- Use `eprintln!` to write to stderr
- Test with small images first
- Verify file permissions

## 🚀 Performance Optimization

### Profiling
```bash
cargo build --release
cargo run --release -- --help
```

### Memory Optimization
- Use `Vec::with_capacity()` for known sizes
- Consider streaming for very large images
- Profile memory usage with tools like `valgrind`

### Speed Optimization
- Use parallel processing with `rayon` for CPU-intensive effects
- Optimize inner loops
- Consider SIMD operations for pixel manipulation

## 📦 Release Process

### Version Management
1. Update `Cargo.toml` version
2. Update documentation
3. Test thoroughly
4. Create git tag
5. Build release binary

### Distribution
```bash
cargo build --release
```

## 🤝 Contributing Guidelines

### Code Review Checklist
- [ ] Code follows style guidelines
- [ ] Effect works with test images
- [ ] Documentation updated
- [ ] No breaking changes to existing effects
- [ ] Error handling appropriate
- [ ] Memory usage reasonable

### Pull Request Process
1. Fork repository
2. Create feature branch
3. Implement changes
4. Test thoroughly
5. Update documentation
6. Submit pull request

---

*Happy databending! 📸💣*