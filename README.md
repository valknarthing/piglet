# 🐷 Piglet

<div align="center">

**Animated and colorful figlet wrapper with motion effects**

[![CI](https://github.com/valknarthing/piglet/workflows/CI/badge.svg)](https://github.com/valknarthing/piglet/actions/workflows/ci.yml)
[![Security Audit](https://github.com/valknarthing/piglet/workflows/Security%20Audit/badge.svg)](https://github.com/valknarthing/piglet/actions/workflows/security.yml)
[![Coverage](https://github.com/valknarthing/piglet/workflows/Coverage/badge.svg)](https://github.com/valknarthing/piglet/actions/workflows/coverage.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/piglet.svg)](https://crates.io/crates/piglet)

</div>

---

## ✨ Features

- 🎨 **Rich Color Support**: CSS4 colors, hex codes, and smooth gradients
- 🎬 **20+ Motion Effects**: Fade, slide, scale, typewriter, wave, bounce, and more
- ⚡ **18+ Easing Functions**: Linear, quad, cubic, elastic, back, bounce variations
- 🖼️ **Figlet Integration**: Full support for figlet fonts and options
- 🔄 **Looping Animations**: Infinite or single-run modes
- 🎯 **High Performance**: Async rendering with configurable FPS
- 🌈 **Gradient Engine**: Parse and apply CSS-style gradients
- 📦 **Cross-Platform**: Linux, macOS, and Windows support

## 📦 Installation

### From Source

```bash
git clone https://github.com/valknarthing/piglet.git
cd piglet
cargo build --release
```

The binary will be available at `target/release/piglet`.

### Prerequisites

Piglet requires `figlet` to be installed on your system:

```bash
# Ubuntu/Debian
sudo apt-get install figlet

# macOS
brew install figlet

# Windows
choco install figlet -y
```

## 🚀 Quick Start

```bash
# Simple color palette animation
piglet "Hello World" -p "#FF5733,#33FF57,#3357FF"

# Gradient with fade-in effect
piglet "Rainbow" -g "linear-gradient(90deg, red, orange, yellow, green, blue, purple)" -e fade-in

# Typewriter effect with custom timing
piglet "Type It Out" -e typewriter -d 3s -i ease-out

# Bouncing text with loop
piglet "Bounce!" -e bounce-in -l
```

## 📖 Usage

```
piglet [TEXT] [OPTIONS]

Arguments:
  <TEXT>  Text to render with figlet

Options:
  -d, --duration <DURATION>        Duration of animation [default: 3s]
                                   Formats: 3000ms, 0.3s, 5m, 0.5h

  -p, --color-palette <COLORS>     Color palette (comma-separated)
                                   Example: "#FF5733,#33FF57,blue,red"

  -g, --color-gradient <GRADIENT>  CSS gradient definition
                                   Example: "linear-gradient(90deg, red, blue)"

  -e, --motion-effect <EFFECT>     Motion effect to apply [default: fade-in]

  -i, --motion-ease <EASING>       Easing function [default: ease-in-out]

  -f, --font <FONT>                Figlet font to use

  -l, --loop                       Loop animation infinitely

      --fps <FPS>                  Frame rate [default: 30]

      --list-effects               List all available effects
      --list-easing                List all available easing functions
      --list-colors                List all CSS4 color names

  -- <FIGLET_ARGS>...              Additional figlet options
                                   Example: -- -w 200 -c

  -h, --help                       Print help
  -V, --version                    Print version
```

## 🎬 Motion Effects

| Effect | Description | Effect | Description |
|--------|-------------|--------|-------------|
| `fade-in` | Fade from transparent | `fade-out` | Fade to transparent |
| `fade-in-out` | Fade in then out | `slide-in-top` | Slide from top |
| `slide-in-bottom` | Slide from bottom | `slide-in-left` | Slide from left |
| `slide-in-right` | Slide from right | `scale-up` | Scale from small |
| `scale-down` | Scale from large | `pulse` | Pulsing effect |
| `bounce-in` | Bounce into view | `bounce-out` | Bounce out of view |
| `typewriter` | Type character by character | `typewriter-reverse` | Untype backwards |
| `wave` | Wave motion | `jello` | Jello wobble |
| `color-cycle` | Cycle through colors | `rainbow` | Rainbow effect |
| `gradient-flow` | Flowing gradient | `rotate-in` | Rotate into view |
| `rotate-out` | Rotate out of view | | |

## ⚡ Easing Functions

| Category | Functions |
|----------|-----------|
| **Linear** | `linear` |
| **Basic** | `ease-in`, `ease-out`, `ease-in-out` |
| **Quadratic** | `ease-in-quad`, `ease-out-quad`, `ease-in-out-quad` |
| **Cubic** | `ease-in-cubic`, `ease-out-cubic`, `ease-in-out-cubic` |
| **Back** | `ease-in-back`, `ease-out-back`, `ease-in-out-back` |
| **Elastic** | `ease-in-elastic`, `ease-out-elastic`, `ease-in-out-elastic` |
| **Bounce** | `ease-in-bounce`, `ease-out-bounce`, `ease-in-out-bounce` |

## 🎨 Color Options

### Palette Mode
Provide a comma-separated list of colors:
```bash
piglet "Text" -p "red,blue,green"
piglet "Text" -p "#FF5733,#33FF57,#3357FF"
piglet "Text" -p "crimson,gold,navy"
```

### Gradient Mode
Use CSS gradient syntax:
```bash
piglet "Text" -g "linear-gradient(90deg, red, blue)"
piglet "Text" -g "linear-gradient(to right, #FF5733 0%, #33FF57 50%, #3357FF 100%)"
```

Supports:
- Hex colors (`#FF5733`)
- CSS4 color names (`red`, `blue`, `crimson`, etc.)
- Position percentages (`0%`, `50%`, `100%`)
- Angle notation (`90deg`, `180deg`, `to right`, `to bottom`)

## 💡 Examples

### Basic Animation
```bash
piglet "Welcome" -e fade-in -d 2s
```

### Rainbow Gradient with Typewriter
```bash
piglet "Rainbow Text" \
  -g "linear-gradient(90deg, red, orange, yellow, green, blue, indigo, violet)" \
  -e typewriter \
  -d 4s \
  -i ease-in-out
```

### Bouncing Logo with Custom Font
```bash
piglet "LOGO" \
  -f slant \
  -e bounce-in \
  -p "#FF6B6B,#4ECDC4,#45B7D1" \
  -i ease-out-bounce \
  -l
```

### Infinite Wave with Gradient Flow
```bash
piglet "Ocean Waves" \
  -g "linear-gradient(180deg, #0077be, #00c9ff, #0077be)" \
  -e wave \
  -l
```

### Custom Figlet Options
```bash
# Center text with width 200
piglet "Centered" -- -w 200 -c

# Use specific font with kerning
piglet "Custom" -f banner -- -k
```

## 🏗️ Architecture

```
CLI Input → Figlet Wrapper → Parser (duration/colors/gradients)
    ↓
Color Engine (palette/gradient interpolation)
    ↓
Animation Engine (effects + easing)
    ↓
Terminal Manager → Render Loop → Output
```

### Key Components
- **FigletWrapper**: Executes figlet and captures ASCII output
- **Parser**: Converts CLI strings to structured data (duration, colors, gradients)
- **ColorEngine**: Manages color palettes and gradient interpolation
- **AnimationEngine**: Applies motion effects with easing functions
- **TerminalManager**: Handles terminal setup/cleanup and frame rendering

## 🔧 Development

### Build
```bash
cargo build
```

### Test
```bash
cargo test --all-features
```

### Lint
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Documentation
```bash
cargo doc --no-deps --all-features
```

## 🎯 Cross-Platform Support

Piglet builds on:
- **Linux**: `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`
- **macOS**: `x86_64-apple-darwin`, `aarch64-apple-darwin`
- **Windows**: `x86_64-pc-windows-msvc`

```bash
# Build for specific target
cargo build --release --target x86_64-unknown-linux-musl
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-effect`)
3. Commit your changes (`git commit -m 'Add amazing effect'`)
4. Push to the branch (`git push origin feature/amazing-effect`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [figlet](http://www.figlet.org/) - The original ASCII art generator
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [palette](https://github.com/Ogeon/palette) - Color space conversions
- [scirs2-interpolate](https://github.com/scirs/scirs2-interpolate) - Easing functions

## 📊 Project Stats

![GitHub code size](https://img.shields.io/github/languages/code-size/valknarthing/piglet)
![GitHub repo size](https://img.shields.io/github/repo-size/valknarthing/piglet)
![Lines of code](https://img.shields.io/tokei/lines/github/valknarthing/piglet)

---

<div align="center">
Made with ❤️ and Rust
</div>
