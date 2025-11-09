# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Piglet is an animated and colorful figlet wrapper written in Rust that renders ASCII art with motion effects, color gradients, and easing functions. It wraps the `figlet` command-line tool to generate ASCII art and applies real-time terminal animations using crossterm.

## Prerequisites

- **figlet** must be installed on the system:
  - Ubuntu/Debian: `sudo apt-get install figlet`
  - macOS: `brew install figlet`
  - Windows: `choco install figlet -y`
- Rust toolchain (managed via rustup)

## Common Commands

### Development
```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the binary
cargo run -- "Hello" -p "#FF5733,#33FF57"

# Run with specific effect
cargo run -- "World" -g "linear-gradient(90deg, red, blue)" -e fade-in
```

### Testing
```bash
# Run all tests (requires figlet installed)
cargo test --verbose --all-features

# Run tests without default features
cargo test --verbose --no-default-features

# Run a single test
cargo test test_figlet_wrapper

# Run integration tests only
cargo test --test integration_tests
```

### Linting and Code Quality
```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all

# Run clippy with all warnings as errors
cargo clippy --all-targets --all-features -- -D warnings

# Check documentation
cargo doc --no-deps --all-features
```

### Cross-compilation Targets
The project supports multiple targets (see CI configuration):
- `x86_64-unknown-linux-gnu`
- `x86_64-unknown-linux-musl` (requires musl-tools)
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`

```bash
# Build for specific target
cargo build --release --target x86_64-unknown-linux-musl
```

## Architecture

### Module Structure

**Core Pipeline Flow:**
1. **CLI Parsing** (`cli.rs`) → Parses command-line arguments using clap
2. **Figlet Wrapper** (`figlet.rs`) → Executes figlet command to generate base ASCII art
3. **Parsers** (`parser/`) → Parses duration, colors, and gradients from CLI strings
4. **Color Engine** (`color/`) → Manages palettes and gradients, interpolates colors
5. **Animation Engine** (`animation/`) → Applies motion effects and easing functions
6. **Terminal Manager** (`utils/terminal.rs`) → Handles terminal setup, rendering, and cleanup

### Key Components

**figlet.rs - FigletWrapper**
- Wraps external `figlet` binary execution
- Validates figlet installation via `which` crate
- Supports custom fonts and arguments passed through to figlet

**parser/ - Input Parsers**
- `duration.rs`: Parses time strings (e.g., "3000ms", "0.3s", "5m", "0.5h") to milliseconds using regex
- `color.rs`: Parses hex colors (#FF5733) and CSS4 color names (red, blue) using csscolorparser
- `gradient.rs`: Parses CSS gradient syntax (e.g., "linear-gradient(90deg, red, blue)")

**color/ - Color System**
- `palette.rs`: Manages discrete color palettes (comma-separated colors)
- `gradient.rs`: Generates smooth color transitions using gradient definitions
- `apply.rs`: Applies colors to ASCII art characters
- Uses `palette` crate for color space conversions and interpolation

**animation/ - Animation System**
- `easing.rs`: Implements 18+ easing functions (linear, ease-in/out, quad, cubic, back, elastic, bounce) using scirs2-interpolate
- `effects/`: Motion effects including fade-in/out, slide, scale, typewriter, wave, rainbow
- `renderer.rs`: Frame-by-frame rendering loop with tokio async timing
- `timeline.rs`: Manages animation progress and frame timing

**utils/ - Utilities**
- `terminal.rs`: Terminal manipulation using crossterm (alternate screen, cursor hiding, clearing)
- `ascii.rs`: ASCII art data structure and character-level manipulation

### External Dependencies

**Critical dependencies:**
- `clap` (4.4+): CLI argument parsing with derive macros
- `crossterm` (0.27): Terminal manipulation and ANSI escape sequences
- `tokio` (1.35): Async runtime for frame timing
- `scirs2-interpolate` (0.1): Easing function implementations
- `palette` (0.7): Color space conversions and interpolation
- `csscolorparser` (0.6): Parsing hex and CSS color names
- `which` (5.0): Locating figlet binary in PATH
- `nom` (7.1): Parser combinators for gradient syntax
- `regex` (1.10): Duration string parsing

### Animation Flow

1. Parse CLI args → validate duration, colors, effect, easing
2. Execute figlet → capture ASCII art output as String
3. Parse colors → build ColorEngine with palette or gradient
4. Create AnimationEngine → configure effect + easing + colors
5. Setup terminal → enter alternate screen, hide cursor
6. Render loop → for each frame:
   - Calculate progress (0.0 to 1.0)
   - Apply easing function to progress
   - Apply effect transformation to ASCII art
   - Apply color at current progress
   - Render to terminal
   - Sleep until next frame (based on fps)
7. Cleanup → restore terminal state

### Testing Strategy

- Unit tests embedded in modules (e.g., `parser/duration.rs`, `figlet.rs`)
- Integration tests in `tests/integration_tests.rs` testing full pipeline
- Tests validate: parsers, color interpolation, easing functions, effects, gradient rendering
- CI runs tests on Ubuntu, macOS, Windows with stable and beta Rust

## CLI Usage Examples

```bash
# Simple color palette
piglet "Hello" -p "#FF5733,#33FF57"

# Gradient with motion effect
piglet "World" -g "linear-gradient(90deg, red, blue)" -e fade-in

# Typewriter with custom duration and easing
piglet "Cool!" -e typewriter -d 2s -i ease-out

# Loop animation infinitely
piglet "Loop" -e wave -l

# Custom figlet font and arguments
piglet "Custom" -f slant -- -w 200 -c

# List available effects/easing/colors
piglet --list-effects
piglet --list-easing
piglet --list-colors
```

## Available Effects

fade-in, fade-out, fade-in-out, slide-in-top, slide-in-bottom, slide-in-left, slide-in-right, scale-up, scale-down, pulse, bounce-in, bounce-out, typewriter, typewriter-reverse, wave, jello, color-cycle, rainbow, gradient-flow, rotate-in, rotate-out

## Available Easing Functions

linear, ease-in, ease-out, ease-in-out, ease-in-quad, ease-out-quad, ease-in-out-quad, ease-in-cubic, ease-out-cubic, ease-in-out-cubic, ease-in-back, ease-out-back, ease-in-out-back, ease-in-elastic, ease-out-elastic, ease-in-out-elastic, ease-in-bounce, ease-out-bounce, ease-in-out-bounce
