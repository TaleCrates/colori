colori
======

A rust crate for simple color conversion and manipulation.

## Features

- [ ]  Color Spaces + Conversion
  - [x]  RGB
  - [x]  HSL
  - [ ]  HSV
  - [ ]  XYZ
  - [ ]  L*a*b
- [ ]  Color Operations
  - [ ]  Basic operations on spaces (lightness, saturation, greyscale etc.)
  - [ ]  Mixing and mixing modes
- [ ]  Color Parsing & Output
  - [ ]  Hex & Integer (RGB)
  - [ ]  Function Expressions
  - [ ]  CSS Compatible output
- [x]  Color List (over 800 named color constants)

## Install

Add the following to your `[dependencies]` in `Cargo.toml`

```toml
colori = "0.1"
```

## Usage

Data structures for different color spaces

```rust
use colori::{RgbColor, HslColor};

let rgb = RgbColor(255, 0, 80);
rgb.red()    // 255
rgb.green()  // 0
rgb.blue()   // 80


let hsl = HslColor(0.5, 0.4, 1.0);
hsl.hue();       // 0.5
hsl.hue_deg()    // 180
hsl.lightness()  // 0.4
hsl.saturation() // 1.0
```

Convert different color spaces into each other

```rust
use colori::{RgbColor, HslColor};

let hsl: HslColor = RgbColor(255, 0, 0).into();

let rgb: RgbColor = HslColor(0.5, 0.4, 1.0).into();
```

Access a list of over 800 defined color constants

```rust
use colori::{Color};

let rgb = Color::UNITED_NATIONS_BLUE;

println!("R: {}, G: {}, B: {}", rgb.red(), rgb.green(), rgb.blue());
```