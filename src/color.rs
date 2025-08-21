// lib.rs
use std::fmt::{self, Display, Formatter};

/// Predefined color constants for common colors
pub mod colors {
    pub const RED: (u8, u8, u8) = (255, 0, 0);
    pub const GREEN: (u8, u8, u8) = (0, 255, 0);
    pub const BLUE: (u8, u8, u8) = (0, 0, 255);
    pub const YELLOW: (u8, u8, u8) = (255, 255, 0);
    pub const CYAN: (u8, u8, u8) = (0, 255, 255);
    pub const MAGENTA: (u8, u8, u8) = (255, 0, 255);
    pub const WHITE: (u8, u8, u8) = (255, 255, 255);
    pub const BLACK: (u8, u8, u8) = (0, 0, 0);
    pub const ORANGE: (u8, u8, u8) = (255, 165, 0);
    pub const PURPLE: (u8, u8, u8) = (128, 0, 128);
    pub const PINK: (u8, u8, u8) = (255, 192, 203);
    pub const BROWN: (u8, u8, u8) = (165, 42, 42);
}

/// Text styling options
#[derive(Clone, Debug, PartialEq)]
pub enum TextStyle {
    Bold,
    Dim,
    Italic,
    Blink,
    Reverse,
    Hidden,
    Reset,
}

/// Underline styles supported
#[derive(Clone, Debug, PartialEq)]
pub enum UnderlineStyle {
    Normal,
    Strikethrough,
    Double,
    Curly,
    Dotted,
    Dashed,
    None,
}

/// Background color support
#[derive(Clone, Debug, PartialEq)]
pub struct BackgroundColor(pub u8, pub u8, pub u8);

/// A comprehensive struct for styled text printing
#[derive(Clone, Debug)]
pub struct StyledText<'a> {
    text: &'a str,
    foreground: Option<(u8, u8, u8)>,
    background: Option<BackgroundColor>,
    underline: UnderlineStyle,
    styles: Vec<TextStyle>,
}

impl<'a> StyledText<'a> {
    /// Create a new styled text instance
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            foreground: None,
            background: None,
            underline: UnderlineStyle::None,
            styles: Vec::new(),
        }
    }

    /// Set foreground color using RGB values
    pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.foreground = Some((r, g, b));
        self
    }

    /// Set foreground color using hex string (e.g., "#FF0000" or "FF0000")
    pub fn hex_color(mut self, hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Hex color must be 6 characters long".to_string());
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;
        
        self.foreground = Some((r, g, b));
        Ok(self)
    }

    /// Set background color
    pub fn bg_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background = Some(BackgroundColor(r, g, b));
        self
    }

    /// Set background color using hex string
    pub fn bg_hex_color(mut self, hex: &str) -> Result<Self, String> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Hex color must be 6 characters long".to_string());
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;
        
        self.background = Some(BackgroundColor(r, g, b));
        Ok(self)
    }

    /// Set underline style
    pub fn underline(mut self, style: UnderlineStyle) -> Self {
        self.underline = style;
        self
    }

    /// Add text style (can be chained for multiple styles)
    pub fn style(mut self, style: TextStyle) -> Self {
        if !self.styles.contains(&style) {
            self.styles.push(style);
        }
        self
    }

    /// Make text bold
    pub fn bold(self) -> Self {
        self.style(TextStyle::Bold)
    }

    /// Make text italic
    pub fn italic(self) -> Self {
        self.style(TextStyle::Italic)
    }

    /// Make text dim
    pub fn dim(self) -> Self {
        self.style(TextStyle::Dim)
    }

    /// Make text blink
    pub fn blink(self) -> Self {
        self.style(TextStyle::Blink)
    }

    /// Reverse colors
    pub fn reverse(self) -> Self {
        self.style(TextStyle::Reverse)
    }

    /// Hide text
    pub fn hidden(self) -> Self {
        self.style(TextStyle::Hidden)
    }

    /// Create a gradient effect across the text
    pub fn gradient(text: &'a str, start_color: (u8, u8, u8), end_color: (u8, u8, u8)) -> String {
        if text.is_empty() {
            return String::new();
        }

        let len = text.chars().count() as f32;
        let mut result = String::new();
        
        for (i, ch) in text.chars().enumerate() {
            let ratio = i as f32 / (len - 1.0).max(1.0);
            let r = (start_color.0 as f32 + (end_color.0 as f32 - start_color.0 as f32) * ratio) as u8;
            let g = (start_color.1 as f32 + (end_color.1 as f32 - start_color.1 as f32) * ratio) as u8;
            let b = (start_color.2 as f32 + (end_color.2 as f32 - start_color.2 as f32) * ratio) as u8;
            
            result.push_str(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, ch));
        }
        result.push_str("\x1b[0m");
        result
    }

    /// Create polychrome text effect
    pub fn polychrome(text: &'a str) -> String {
        if text.is_empty() {
            return String::new();
        }

        let rainbow_colors = [
            (255, 0, 0),   // Red
            (255, 165, 0), // Orange
            (255, 255, 0), // Yellow
            (0, 255, 0),   // Green
            (0, 0, 255),   // Blue
            (75, 0, 130),  // Indigo
            (238, 130, 238), // Violet
        ];

        let len = text.chars().count();
        let mut result = String::new();
        
        for (i, ch) in text.chars().enumerate() {
            let color_index = (i * rainbow_colors.len()) / len;
            let color = rainbow_colors[color_index.min(rainbow_colors.len() - 1)];
            result.push_str(&format!("\x1b[38;2;{};{};{}m{}", color.0, color.1, color.2, ch));
        }
        result.push_str("\x1b[0m");
        result
    }
}

impl<'a> Display for StyledText<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut codes = Vec::new();

        // Add foreground color
        if let Some((r, g, b)) = self.foreground {
            codes.push(format!("\x1b[38;2;{};{};{}m", r, g, b));
        }

        // Add background color
        if let Some(BackgroundColor(r, g, b)) = &self.background {
            codes.push(format!("\x1b[48;2;{};{};{}m", r, g, b));
        }

        // Add text styles
        for style in &self.styles {
            let code = match style {
                TextStyle::Bold => "\x1b[1m",
                TextStyle::Dim => "\x1b[2m",
                TextStyle::Italic => "\x1b[3m",
                TextStyle::Blink => "\x1b[5m",
                TextStyle::Reverse => "\x1b[7m",
                TextStyle::Hidden => "\x1b[8m",
                TextStyle::Reset => "\x1b[0m",
            };
            codes.push(code.to_string());
        }

        // Add underline
        let underline_code = match self.underline {
            UnderlineStyle::Normal => "\x1b[4m",
            UnderlineStyle::Strikethrough => "\x1b[9m",
            UnderlineStyle::Double => "\x1b[21m",
            UnderlineStyle::Curly => "\x1b[4:3m",
            UnderlineStyle::Dotted => "\x1b[4:4m",
            UnderlineStyle::Dashed => "\x1b[4:5m",
            UnderlineStyle::None => "",
        };

        if !underline_code.is_empty() {
            codes.push(underline_code.to_string());
        }

        // Write the styled text
        write!(f, "{}{}\x1b[0m", codes.join(""), self.text)
    }
}

/// Extension trait for easy styling
pub trait StyleExt {
    /// Convert to StyledText for further styling
    fn styled(&self) -> StyledText<'_>;
    
    /// Quick color application
    fn color(&self, r: u8, g: u8, b: u8) -> StyledText<'_>;
    
    /// Quick hex color application
    fn hex_color(&self, hex: &str) -> Result<StyledText<'_>, String>;
    
    /// Quick bold styling
    fn bold(&self) -> StyledText<'_>;
    
    /// Quick italic styling
    fn italic(&self) -> StyledText<'_>;
    
    /// Quick underline styling
    fn underline(&self, style: UnderlineStyle) -> StyledText<'_>;
}

impl StyleExt for str {
    fn styled(&self) -> StyledText<'_> {
        StyledText::new(self)
    }

    fn color(&self, r: u8, g: u8, b: u8) -> StyledText<'_> {
        StyledText::new(self).color(r, g, b)
    }

    fn hex_color(&self, hex: &str) -> Result<StyledText<'_>, String> {
        StyledText::new(self).hex_color(hex)
    }

    fn bold(&self) -> StyledText<'_> {
        StyledText::new(self).bold()
    }

    fn italic(&self) -> StyledText<'_> {
        StyledText::new(self).italic()
    }

    fn underline(&self, style: UnderlineStyle) -> StyledText<'_> {
        StyledText::new(self).underline(style)
    }
}

/// Utility functions for terminal styling
pub mod utils {
    /// Clear the terminal screen
    pub fn clear_screen() {
        print!("\x1b[2J\x1b[H");
    }

    /// Move cursor to position (row, col)
    pub fn move_cursor(row: u16, col: u16) {
        print!("\x1b[{};{}H", row, col);
    }

    /// Hide cursor
    pub fn hide_cursor() {
        print!("\x1b[?25l");
    }

    /// Show cursor
    pub fn show_cursor() {
        print!("\x1b[?25h");
    }

    /// Reset all terminal formatting
    pub fn reset_formatting() {
        print!("\x1b[0m");
    }

    /// Check if terminal supports colors
    pub fn supports_color() -> bool {
        std::env::var("NO_COLOR").is_err() && 
        (std::env::var("FORCE_COLOR").is_ok() || 
         std::env::var("TERM").map_or(false, |term| !term.is_empty() && term != "dumb"))
    }
}

/// Progress bar component
pub struct ProgressBar {
    width: usize,
    filled_char: char,
    empty_char: char,
    color: Option<(u8, u8, u8)>,
}

impl ProgressBar {
    /// Create a new progress bar
    pub fn new(width: usize) -> Self {
        Self {
            width,
            filled_char: '█',
            empty_char: '░',
            color: None,
        }
    }

    /// Set the characters used for filled and empty parts
    pub fn chars(mut self, filled: char, empty: char) -> Self {
        self.filled_char = filled;
        self.empty_char = empty;
        self
    }

    /// Set the color of the progress bar
    pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = Some((r, g, b));
        self
    }

    /// Render the progress bar with a given progress (0.0 to 1.0)
    pub fn render(&self, progress: f64) -> String {
        let progress = progress.clamp(0.0, 1.0);
        let filled_width = (self.width as f64 * progress) as usize;
        let empty_width = self.width - filled_width;

        let bar = format!("{}{}",
            self.filled_char.to_string().repeat(filled_width),
            self.empty_char.to_string().repeat(empty_width)
        );

        if let Some((r, g, b)) = self.color {
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, bar)
        } else {
            bar
        }
    }
}