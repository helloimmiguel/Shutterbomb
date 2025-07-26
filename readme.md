# 📸💣 Shutterbomb - Interactive Databending Tool

A terminal-based image databending application inspired by the work of Jack Stauber and Tally Hall. Made by a photographer to other photographers with love, Rust and Copilot Pro ^-^

## 🎯 Overview

Shutterbomb is an interactive TUI (Terminal User Interface) application that allows you to apply creative databending effects to images. Databending is the practice of manipulating data files in a way they're not meant to, in order to create artistic glitches and visual effects. This effect is described poetically in the song "Databend" by Jack Stauber in the album "HiLo" which was the catalyst and the motor for all of this, but almost all of his audiovisual work features databending in some form, be it musical or visual.

## ✨ Features

- **Interactive Terminal Interface**: Navigate with keyboard controls
- **7 Unique Effects**: Each with its own artistic style
- **Real-time Parameter Editing**: Customize effect parameters
- **Progress Indication**: Visual feedback during processing
- **Cross-platform**: Works on Windows and can be compiled to work on macOS, and Linux (and any potato that runs Rust)

## 🎨 Available Effects

### 📸 Oversensibility
Produces an effect similar to high‑ISO digital photos or film, introducing random noise that mimics the look of heavy grain. The process works by giving each pixel a probability set by the “ISO” parameter of being altered. When a pixel is selected, its value is replaced with a randomly chosen one, creating a databent, grainy appearance.
- **Parameter**: ISO (0-9999)
- **Effect**: Adds random noise based on a virtual "ISO" value

### ☀️ Overexposure  
Produces an effect similar to cranking the exposure up on pictures, randomly brighting up pixels to simulate the look of a blown-out image while introducing noise. The process works by giving a random chance of being brighten by the exposure factor. 
- **Parameter**: Exposure Factor (0.1-3.0)
- **Effect**: Randomly brightens pixels to simulate overexposure

### 🎹 Synesthesia
This mode gives the user liberty to databent the image themself, by pressing keys on the keyboard that bent a random amount of pixels using a random value. It's inspired on the human condition of mixing up senses, like sight and touch.
- **Parameters**: None (interactive)
- **Effect**: Real-time manipulation based on keystrokes

### ☁️ Variations on a Cloud
Creates glitchy patches by copying random image regions. Inspired on the art cover by Tally Hall under the moniker of Miracle Musical for the song "Variations on a Cloud". The art cover consists of an image of a cloud cut in little squares and mixed up (see the image below). 
![cover](https://static.wikia.nocookie.net/tallyhall/images/2/28/Variations_on_a_Cloud_Album_Cover.png/revision/latest/scale-to-width-down/268?cb=20150728023730)
- **Parameter**: Patch Size (10-200)

### ⚡ The Mind Electric
Based on the insanity of the song "The Mind Electric" also made by Miracle Musical, this effects applies a classic glitch art effect to an image using offset layers with color shifts to create images akin to the mind of those who went through shock-therapy
- **Parameter**: Layers (1-20)
- **Effect**: Applies multiple offset layers with color shifts

### 🎵 Jack Stauberism
Works by coping over and over the lyrics of the song "Databend" by Jack Stauber translated to proper values into the raw pixel buffer of the image as the user presses keys, creating an poetic effect of databending, mixing two kinds of poetry
- **Parameters**: None (interactive)
- **Effect**: Uses song lyrics to corrupt image data

### 🌈 New Normal
Based on the song and short film "New Normal", it creates a new reality for the image by databending 1/16th of the image each time the user presses a key, bending its reality like an sudden depression and reversal of the meaning of "normal".

- **Parameters**: None (interactive)
- **Effect**: Real-time random corruption based on input

## 🚀 Installation

### Prerequisites
- Rust (2024)
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
|── README.md            # This file
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
- **File formats**: Currently tested on JPEGs but other lossy and RAW image formats such a PNGs and CR2s are planned.
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

---

*"🎵 I've began to databend 🎵"*