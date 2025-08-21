# <h1 align="center">🎨 Polychrome</h1>

<p align="center">
  <img src="https://raw.githubusercontent.com/EliasLeguizamon123/polychrome/main/logo.png?raw=true" alt="Polychrome Logo" style="width:300px;"/>
</p>

<p align="center">
  <strong>A modern, feature-rich terminal styling library for Rust</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/polychrome">
    <img src="https://img.shields.io/crates/v/polychrome.svg" alt="Crates.io">
  </a>
  <a href="https://docs.rs/polychrome">
    <img src="https://docs.rs/polychrome/badge.svg" alt="Documentation">
  </a>
  <img src="https://img.shields.io/crates/l/polychrome.svg" alt="License">
</p>

---

## ✨ Features

- 🌈 **RGB and Hex Colors**: Full truecolor support with easy-to-use APIs
- 🎨 **Background Colors**: Style text with colorful backgrounds
- **📝 Text Styling**: Bold, italic, dim, blink, and more
- 🌊 **Gradient Effects**: Create beautiful color transitions
- 🌈 **Polychrome Text**: Automatic polychrome coloring
- 📊 **Progress Bars**: Built-in customizable progress indicators
- 🖥️ **Terminal Utils**: Cursor control, screen clearing, and more
- 🚀 **Zero Dependencies**: Lightweight and fast
- 🔗 **Chainable API**: Intuitive method chaining
- 🦀 **Memory Safe**: Built with Rust's safety guarantees

## 🚀 Quick Start

Add Polychrome to your `Cargo.toml`:

```toml
[dependencies]
polychrome = "3.0.0"
```

Then start styling your terminal output:

```rust
use polychrome::{StyleExt, colors};

fn main() {
    // Simple colored text
    println!("{}", "Hello, world!".color(255, 0, 0));
    
    // Hex colors
    println!("{}", "Hex colors!".hex_color("#00FF00").unwrap());
    
    // Chained styling
    println!("{}", 
        "Bold and colorful!"
            .styled()
            .bold()
            .color(colors::BLUE.0, colors::BLUE.1, colors::BLUE.2)
    );
}
```

## 📖 Examples

### Basic Text Styling

```rust
use polychrome::{StyleExt, UnderlineStyle};

// Colors and styles
println!("{}", "Bold red text".color(255, 0, 0).bold());
println!("{}", "Italic blue".color(0, 0, 255).italic());
println!("{}", "Underlined".underline(UnderlineStyle::Normal));

// Background colors
println!("{}", 
    "White on red background"
        .styled()
        .color(255, 255, 255)
        .bg_color(255, 0, 0)
);
```

### Advanced Effects

```rust
use polychrome::{StyledText, colors};

// Gradient text
let gradient = StyledText::gradient("GRADIENT TEXT", colors::RED, colors::BLUE);
println!("{}", gradient);

// Polychrome effect
let polychrome = StyledText::polychrome("🌈 POLYCHROME TEXT 🌈");
println!("{}", polychrome);
```

### Progress Bars

```rust
use polychrome::ProgressBar;

let bar = ProgressBar::new(30).color(0, 255, 0);
println!("[{}] 75%", bar.render(0.75));

// Custom characters
let custom_bar = ProgressBar::new(20).chars('▓', '▒');
println!("[{}] 50%", custom_bar.render(0.5));
```

### Predefined Colors

```rust
use polychrome::colors;

println!("{}", "Red".color(colors::RED.0, colors::RED.1, colors::RED.2));
println!("{}", "Green".color(colors::GREEN.0, colors::GREEN.1, colors::GREEN.2));
println!("{}", "Blue".color(colors::BLUE.0, colors::BLUE.1, colors::BLUE.2));
```

Available colors: `RED`, `GREEN`, `BLUE`, `YELLOW`, `CYAN`, `MAGENTA`, `WHITE`, `BLACK`, `ORANGE`, `PURPLE`, `PINK`, `BROWN`

### Terminal Utilities

```rust
use polychrome::utils;

// Clear screen and move cursor
utils::clear_screen();
utils::move_cursor(10, 5);

// Hide/show cursor
utils::hide_cursor();
utils::show_cursor();

// Check color support
if utils::supports_color() {
    println!("Terminal supports colors!");
}
```

## 🎨 Underline Styles

```rust
use polychrome::UnderlineStyle;

println!("{}", "Normal".underline(UnderlineStyle::Normal));
println!("{}", "Strike".underline(UnderlineStyle::Strikethrough));
println!("{}", "Double".underline(UnderlineStyle::Double));
println!("{}", "Curly".underline(UnderlineStyle::Curly));
println!("{}", "Dotted".underline(UnderlineStyle::Dotted));
println!("{}", "Dashed".underline(UnderlineStyle::Dashed));
```

## 🔗 Method Chaining

Polychrome supports intuitive method chaining for complex styling:

```rust
use polychrome::{StyleExt, UnderlineStyle, colors};

println!("{}", 
    "Complex styling"
        .styled()
        .hex_color("#FF6B35").unwrap()
        .bg_hex_color("#2E3440").unwrap()
        .bold()
        .italic()
        .underline(UnderlineStyle::Curly)
);
```

## 🖥️ Terminal Compatibility

Polychrome works with modern terminals that support ANSI escape codes and 24-bit colors:

- ✅ **Windows Terminal**
- ✅ **iTerm2** (macOS)
- ✅ **GNOME Terminal** (Linux)
- ✅ **Alacritty**
- ✅ **Kitty**
- ✅ **VS Code Terminal**
- ⚠️ **CMD** (Windows) - Limited support

## 🚀 Performance

Polychrome is designed to be fast and lightweight:
- Zero runtime dependencies
- Minimal memory allocations
- Efficient string formatting
- No heap allocations for basic styling

## 📊 Migration from v2.x

If you're upgrading from Polychrome v2.x:

```rust
// Old v2.x API
use polychrome::ColorPrintExt;
println!("{}", "Hello".color(255, 0, 0).underline(None));

// New v3.0 API (recommended)
use polychrome::{StyleExt, UnderlineStyle};
println!("{}", "Hello".color(255, 0, 0).underline(UnderlineStyle::Normal));
```

## 📚 Documentation

- [API Documentation](https://docs.rs/polychrome)
- [Crates.io Page](https://crates.io/crates/polychrome)
- [GitHub Repository](https://github.com/EliasLeguizamon123/polychrome)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT OR Apache-2.0 license.

---

<p align="center">
  Made with ❤️ and 🦀 by the Rust community
</p>