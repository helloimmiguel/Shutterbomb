# 📸💣 Shutterbomb — Interactive Image Databending

A terminal-based image databending application inspired by the work of Jack Stauber and Tally Hall. Made by a photographer for other photographers with love, Rust and Copilot Pro ^-^

## 🎯 Overview

Shutterbomb is an interactive TUI (Terminal User Interface) application that allows you to apply creative databending effects to images. Databending is the practice of manipulating data files in ways they were not intended to be used, in order to create artistic glitches and visual effects. This effect is described poetically in the song "Databend" by Jack Stauber on the album "HiLo", which was the catalyst for all of this — but almost all of his audiovisual work features databending in some form, be it musical or visual.

## ✨ Features

- **Interactive Terminal Interface**: Navigate with keyboard controls
- **7 Unique Effects**: Each with its own artistic style
- **Real-time Parameter Editing**: Customize effect parameters
- **Context-sensitive Help**: The status bar adapts to your current input mode
- **Progress Indication**: Visual feedback during processing
- **Cross-platform**: Works on Windows, macOS, and Linux

## 🎨 Available Effects

### 📸 Oversensibility
Produces an effect similar to high-ISO digital photos or film grain, introducing random noise. Each pixel has a probability (set by the ISO parameter) of being replaced with a random value, creating a databent, grainy appearance.
- **Parameter**: ISO (0–6400)
- **Effect**: Adds random noise based on a virtual ISO value

### ☀️ Overexposure
Produces an effect similar to cranking up the exposure, randomly brightening pixels to simulate blown-out highlights while introducing noise.
- **Parameter**: Exposure Factor (0.1–3.0)
- **Effect**: Randomly brightens pixels to simulate overexposure

### 🎹 Synesthesia
Gives the user freedom to databend the image by pressing keys on the keyboard. Each keypress bends a random number of pixels using a value derived from the character. Inspired by the human condition of mixing senses, like sight and touch.
- **Parameters**: None (interactive)
- **Effect**: Real-time manipulation based on keystrokes

### ☁️ Variations on a Cloud
Creates glitchy patches by shuffling square regions of the image. Inspired by the album art for the Miracle Musical song "Variations on a Cloud", which consists of a cloud image cut into small squares and rearranged.
- **Parameter**: Patch Size (10–200)
- **Effect**: Shuffles square image regions

### ⚡ The Mind Electric
Based on the intensity of the Miracle Musical song "The Mind Electric", this effect applies layered glitch art with offset layers, color shifts, and alpha blending.
- **Parameter**: Layers (1–20)
- **Effect**: Applies multiple offset layers with color shifts

### 🎵 Jack Stauberism
Copies the lyrics of "Databend" by Jack Stauber — translated into raw byte values — into the pixel buffer as the user presses keys, creating a poetic form of databending that merges textual and visual art.
- **Parameters**: None (interactive)
- **Effect**: Uses song lyrics to corrupt image data

### 🌈 New Normal
Based on the song and short film "New Normal", this effect creates a new reality for the image by randomizing 1/16th of the pixel data each time the user presses a key.
- **Parameters**: None (interactive)
- **Effect**: Real-time random corruption based on input

## 🚀 Installation

### Prerequisites
- Rust (edition 2024)
- Cargo package manager

### Building from Source
```bash
git clone https://github.com/helloimmiguel/Shutterbomb.git
cd Shutterbomb
cargo build --release
```

### Running
```bash
cargo run --release
```

## 🎮 Usage

### Basic Workflow
1. Launch the application with `cargo run`
2. Use arrow keys (↑↓) or vim keys (j/k) to select an effect
3. Press `i` to set the input image path
4. Press `o` to set the output image path
5. Press `p` to edit parameters (if the effect has any)
6. Press `Enter` to execute the effect
7. Press `q` or `Esc` to quit

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

The status bar at the bottom shows your current mode and the available controls:

- **Select Effect**: Navigate and select effects
- **Editing Input Path**: Type the path to your source image
- **Editing Output Path**: Type where to save the processed image
- **Editing Parameters**: Set effect-specific values
- **Processing**: Effect is running (interactive effects accept keypresses)

## 📁 Project Structure

```
Shutterbomb/
├── src/
│   ├── main.rs               # TUI application and event loop
│   └── libdatabend/
│       ├── mod.rs             # Module declarations
│       ├── oversensibility.rs # ISO noise simulation
│       ├── overexposure.rs    # Brightness corruption
│       ├── synestesia.rs      # Interactive key-based bending
│       ├── variationsonacloud.rs # Patch shuffling
│       ├── themindelectric.rs # Alpha blending layers
│       ├── jackstauberism.rs  # Lyrical corruption
│       └── newnormal.rs       # Chaos mode
├── docs/
│   ├── ARCHITECTURE.md        # System architecture
│   ├── DEVELOPMENT.md         # Developer guide
│   ├── EFFECTS_REFERENCE.md   # Detailed effect documentation
│   └── UI_COMPONENTS.md       # TUI component reference
├── Cargo.toml
├── LICENSE
└── readme.md                  # This file
```

## 🔧 Dependencies

- `ratatui` — Terminal user interface framework
- `crossterm` — Cross-platform terminal manipulation
- `image` — Image processing library
- `rand` — Random number generation

## 🎯 Example Usage

1. **Basic Image Corruption**:
   - Select "Oversensibility"
   - Set input: `./input.jpg`
   - Set output: `./corrupted.jpg`
   - Set ISO: `3200`
   - Press Enter

2. **Interactive Databending**:
   - Select "Synesthesia"
   - Set input and output paths
   - Press Enter
   - Type characters to manipulate the image
   - Press Esc when done

## ⚠️ Important Notes

- **Backup your images**: Always work with copies
- **File formats**: Supports any format handled by the `image` crate (JPEG, PNG, BMP, TIFF, etc.)
- **Interactive effects**: Some effects require keyboard input during processing
- **Processing time**: Large images may take longer to process

## 🐛 Troubleshooting

### Common Issues

**"Input file does not exist"**
- Check file path spelling and extension
- Use absolute paths if relative paths don't work

**Application crashes during processing**
- Ensure sufficient memory for large images
- Check that the image file isn't corrupted

**Interactive effects not responding**
- Make sure the terminal has focus
- Press Esc to exit interactive mode

## 🎨 Tips for Best Results

- **Start with lower parameters** for subtle effects
- **Experiment with different effects** on the same image
- **Use high-resolution images** for more dramatic results
- **Save intermediate results** to build layered effects

## 🤝 Contributing

Feel free to submit issues and enhancement requests!

---

*"I've begun to databend"*
