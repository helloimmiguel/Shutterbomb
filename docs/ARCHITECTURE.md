# 🏗️ Architecture Documentation

Detailed architectural overview of the Shutterbomb application.

## 📋 Table of Contents

1. [System Overview](#system-overview)
2. [Module Structure](#module-structure)
3. [Data Flow](#data-flow)
4. [Component Details](#component-details)
5. [Error Handling](#error-handling)
6. [Performance Considerations](#performance-considerations)

## System Overview

Shutterbomb follows a modular architecture with clear separation between the user interface, application logic, and effect processing layers.

```
┌─────────────────┐
│   Terminal UI   │ ← User Interaction Layer
├─────────────────┤
│  App Controller │ ← Application Logic Layer
├─────────────────┤
│ Effect Modules  │ ← Effect Processing Layer
├─────────────────┤
│ Image Libraries │ ← Data Processing Layer
└─────────────────┘
```

## Module Structure

### Core Modules

```
src/
├── main.rs                 # Entry point and TUI implementation
└── libdatabend/           # Effect processing library
    ├── mod.rs             # Module declarations
    ├── oversensibility.rs # ISO noise simulation
    ├── overexposure.rs    # Brightness corruption
    ├── synestesia.rs      # Interactive key-based bending
    ├── variationsonacloud.rs # Patch copying effect
    ├── themindelectric.rs # Alpha blending layers
    ├── jackstauberism.rs  # Lyrical corruption
    └── newnormal.rs       # Chaos mode effect
```

## Data Flow

### 1. User Input Processing

```
Keyboard Input → CrossTerm Events → Event Handler → App State Update → UI Refresh
```

### 2. Effect Execution Flow

```
User Action → Parameter Validation → Image Loading → Effect Processing → Image Saving → Status Update
```

### 3. Interactive Effect Flow

```
Effect Start → Raw Mode Enable → Key Capture → Image Modification → Exit → Raw Mode Disable
```

## Component Details

### Main Application (`main.rs`)

#### Key Structures

**`App` Struct**
```rust
struct App {
    effects: Vec<DatabendEffect>,    // Available effects list
    selected_effect: usize,          // Currently selected effect index
    input_path: String,              // Source image path
    output_path: String,             // Destination image path
    params: Vec<String>,             // Effect parameters
    current_input: InputMode,        // Current input mode state
    processing: bool,                // Processing status flag
    progress: f64,                   // Progress percentage
    status_message: String,          // Status bar message
    last_update: Instant,            // Last update timestamp
    synesthesia_state: Option<SynesthesiaState>, // Interactive synesthesia session
}
```

**`DatabendEffect` Struct**
```rust
struct DatabendEffect {
    name: String,                    // Display name
    description: String,             // Help text
    emoji: String,                   // Icon representation
    params: Vec<String>,             // Parameter descriptions
}
```

**`InputMode` Enum**
```rust
enum InputMode {
    SelectingEffect,                 // Main navigation mode
    InputPath,                       // Input path editing
    OutputPath,                      // Output path editing
    Parameters(usize),               // Parameter editing (with index)
    Processing,                      // Processing state
}
```

#### Key Functions

**`run_app()`**
- Main event loop
- Handles keyboard input with 50ms polling
- Filters for `KeyEventKind::Press` to prevent double registration
- Updates UI and processes events

**`ui()`**
- Renders the complete interface
- Manages layout with constraint-based sizing
- Handles dynamic styling based on application state
- Displays processing overlay when needed

**`execute_effect()`**
- Validates input parameters
- Calls appropriate effect module
- Manages processing state and progress

### Effect Library (`libdatabend/`)

Each effect module follows a consistent interface pattern:

#### Standard Pattern
```rust
pub fn main(input_path: &str, output_path: &str, param: ParamType) {
    // 1. Image loading
    let img = ImageReader::open(input_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    
    // 2. Data conversion
    let mut rawimg = img.to_rgba8().into_raw();
    
    // 3. Effect processing
    // [Effect-specific logic here]
    
    // 4. Image reconstruction and saving
    let new_img: RgbaImage = ImageBuffer::from_raw(img.width(), img.height(), rawimg)
        .expect("Failed to create new image");
    DynamicImage::ImageRgba8(new_img).save(output_path)
        .expect("Failed to save image");
}
```

#### Interactive Pattern
```rust
pub fn main(input_path: &str, output_path: &str) {
    // Load and prepare image
    let mut rawimg = img.to_rgba8().into_raw();
    
    // Enter interactive mode
    enable_raw_mode().expect("failed to enable raw mode");
    
    loop {
        if event::poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => {
                        // Process character input
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }
    
    // Exit interactive mode
    disable_raw_mode().expect("failed to disable raw mode");
    
    // Save result
}
```

## Error Handling

### Current Error Strategy

1. **Fatal Errors**: Use `.expect()` with descriptive messages
2. **Parameter Errors**: Use `.unwrap_or()` with sensible defaults
3. **Interactive Errors**: Return `Result` types for complex operations

## Performance Considerations

### Memory Management

1. **Image Data**: Images converted to `Vec<u8>` for direct manipulation
2. **Cloning Strategy**: Raw image data cloned at effect start
3. **Memory Usage**: 4 bytes per pixel (RGBA format)

### Processing Optimization

1. **Polling Rate**: 50ms for responsive UI without excessive CPU usage
2. **Progress Updates**: 2% increments for smooth progress bars
3. **Batch Processing**: Some effects process pixels in chunks

### Known Performance Issues

1. **Large Images**: No streaming support for very large files
2. **Interactive Effects**: Terminal polling may cause lag
3. **Multiple Layers**: `themindelectric.rs` scales poorly with layer count

## Thread Safety

- **Main Thread**: All UI and effect processing
- **No Concurrency**: Currently single-threaded design
- **Raw Mode**: Terminal state managed carefully to avoid corruption

## Dependencies

### Core Dependencies
- `ratatui`: Terminal UI framework
- `crossterm`: Cross-platform terminal handling
- `image`: Image loading, processing, and saving
- `rand`: Random number generation for effects

### Architectural Implications
- **No async/await**: Synchronous processing model
- **Blocking I/O**: File operations block UI updates
- **Terminal Dependency**: Requires terminal environment

---

*Architecture designed for simplicity and modularity*