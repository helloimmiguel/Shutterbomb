# 📸💣 Shutterbomb - Interactive Databending Tool

A terminal-based image databending application that applies various glitch effects to images through data manipulation.

## 🎯 Overview

Shutterbomb is an interactive TUI (Terminal User Interface) application that allows you to apply creative databending effects to images. Databending is the practice of manipulating data files to create artistic glitches and visual effects.

## ✨ Features

- **Interactive Terminal Interface**: Navigate with keyboard controls
- **7 Unique Effects**: Each with its own artistic style
- **Real-time Parameter Editing**: Customize effect parameters
- **Progress Indication**: Visual feedback during processing
- **Cross-platform**: Works on Windows, macOS, and Linux

## 🎨 Available Effects

### 📸 Oversensibility
Simulates high ISO sensitivity with random noise corruption.
- **Parameter**: ISO (0-6400)
- **Effect**: Adds random noise based on ISO value

### ☀️ Overexposure  
Creates blown-out highlights with random brightness boosts.
- **Parameter**: Exposure Factor (0.1-3.0)
- **Effect**: Randomly brightens pixels to simulate overexposure

### 🎹 Synesthesia
Interactive databending - press keys to bend reality.
- **Parameters**: None (interactive)
- **Effect**: Real-time manipulation based on keystrokes

### ☁️ Variations on a Cloud
Creates glitchy patches by copying random image regions.
- **Parameter**: Patch Size (10-200)
- **Effect**: Copies rectangular regions to random locations

### ⚡ The Mind Electric
Layered chaos with alpha blending and color shifts.
- **Parameter**: Layers (1-20)
- **Effect**: Applies multiple offset layers with color shifts

### 🎵 Jack Stauberism
Lyrical databending with song lyrics as corruption data.
- **Parameters**: None (interactive)
- **Effect**: Uses song lyrics to corrupt image data

### 🌈 New Normal
Interactive chaos mode - embrace the new normal.
- **Parameters**: None (interactive)
- **Effect**: Real-time random corruption based on input

## 🚀 Installation

### Prerequisites
- Rust (latest stable version)
- Cargo package manager

### Building from Source
```bash
git clone <repository-url>
cd Shutterbomb
cargo build --release
```

### Running
```bash
cargo run
```

## 🎮 Usage

### Basic Workflow
1. Launch the application with `cargo run`
2. Use arrow keys (↑↓) or vim keys (j/k) to select an effect
3. Press 'i' to set input image path
4. Press 'o' to set output image path  
5. Press 'p' to edit parameters (if available)
6. Press Enter to execute the effect
7. Press 'q' or Esc to quit

### Controls Reference

| Key | Action |
|-----|--------|
| ↑/↓ or j/k | Navigate effects list |
| i | Edit input path |
| o | Edit output path |
| p | Edit parameters |
| Enter | Execute selected effect |
| Esc | Cancel current input / Exit |
| q | Quit application |

### Input Modes

The application has different input modes indicated by green highlighting:

- **Effect Selection**: Navigate and select effects
- **Input Path**: Type the path to your source image
- **Output Path**: Type where to save the processed image
- **Parameters**: Edit effect-specific parameters

## 📁 Project Structure

```
Shutterbomb/
├── src/
│   ├── main.rs              # Main TUI application
│   └── libdatabend/
│       ├── mod.rs           # Module declarations
│       ├── oversensibility.rs
│       ├── overexposure.rs
│       ├── synestesia.rs
│       ├── variationsonacloud.rs
│       ├── themindelectric.rs
│       ├── jackstauberism.rs
│       └── newnormal.rs
├── docs/
│   ├── README.md            # This file
│   └── libdatabend.md       # Effect library documentation
└── Cargo.toml
```

## 🔧 Dependencies

- `ratatui`: Terminal user interface framework
- `crossterm`: Cross-platform terminal manipulation
- `image`: Image processing library
- `rand`: Random number generation

## 🎯 Example Usage

1. **Basic Image Corruption**:
   - Select "Oversensibility" 
   - Set input: `./input.jpg`
   - Set output: `./corrupted.jpg`
   - Set ISO: `3200`
   - Press Enter

2. **Interactive Databending**:
   - Select "Synesthesia"
   - Set paths
   - Press Enter
   - Type characters to manipulate the image
   - Press Esc when done

## ⚠️ Important Notes

- **Backup your images**: Always work with copies
- **File formats**: Supports common formats (JPEG, PNG, etc.)
- **Interactive effects**: Some effects require keyboard input during processing
- **Processing time**: Large images may take longer to process

## 🐛 Troubleshooting

### Common Issues

**"Input file does not exist"**
- Check file path spelling and extension
- Use absolute paths if relative paths don't work

**Application crashes during processing**
- Ensure sufficient memory for large images
- Check image file isn't corrupted

**Interactive effects not responding**
- Make sure terminal has focus
- Try pressing Esc to exit interactive mode

## 🎨 Tips for Best Results

- **Start with lower parameters** for subtle effects
- **Experiment with different effects** on the same image
- **Use high-resolution images** for more dramatic results
- **Save intermediate results** to build layered effects

## 🤝 Contributing

Feel free to submit issues and enhancement requests!

## 📄 License

[Add your license information here]

---

*"🎵 I've began to databend 🎵"*