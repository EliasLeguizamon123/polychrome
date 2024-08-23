// lib.rs
/// This enum represents the underline style of the text
/// 
/// # Variants
/// - `String`: The underline style is a string (for now only 'stripe' is supported)
/// - `None`: The underline style is none (normal underline)
#[derive(Clone)]
pub enum UnderlineStyle {
    /// The underline style is a string (for now only 'stripe' is supported)
    String(String),
    /// The underline style is none (normal underline)
    None,
}
/// A struct for color printing contains the text, color and underline style
pub struct ColorPrint<'a>(&'a str, Option<(u8, u8, u8)>, UnderlineStyle);

impl<'a> std::fmt::Display for ColorPrint<'a> {
    /// Print the text with color and underline style
    /// 
    /// # Example
    /// 
    /// ```
    /// use color_print::ColorPrintExt;
    /// 
    /// println!("{}", "Hello, world!".color(255, 0, 0).underline(None)); // Color and underline
    /// println!("{}", "Hello, world!".color(0, 0, 255).underline(Some("stripe"))); // Color and striped 
    /// ```

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_code = match self.1 {
            Some((r, g, b)) => format!("\x1b[38;2;{};{};{}m", r, g, b),
            None => "".to_string(),
        };
        let reset_code = "\x1b[0m";
        
        let underline_code = match &self.2 {
            UnderlineStyle::String(style) => match style.as_str() {
                "normal" => "\x1b[4m",
                "stripe" => "\x1b[9m",
                _ => "",
            },
            UnderlineStyle::None => "\x1b[4m",
        };

        write!(f, "{}{}{}{}", color_code, underline_code, self.0, reset_code)
    }
}

pub trait ColorPrintExt {
    /// Set the color of the text
    /// 
    /// # Params
    /// - `r`: Red color value
    /// - `g`: Green color value
    /// - `b`: Blue color value
    /// 
    /// # Example
    /// 
    /// ```
    /// use color_print::ColorPrintExt;
    /// 
    /// println!("{}", "Hello, world!".color(255, 0, 0)); // Red color
    /// ```
    /// 
    fn color<'a>(&'a self, r: u8, g: u8, b: u8) -> ColorPrint<'a>;
    /// Set the underline style of the text
    /// 
    /// # Params
    /// - `style`: The underline style, can be `None`, `stripe`
    /// 
    /// # Example
    /// 
    /// ```
    /// use color_print::ColorPrintExt;
    /// 
    /// println!("{}", "Hello, world!".underline(None)); // Normal underline
    /// println!("{}", "Hello, world!".underline(Some("stripe"))); // Striped underline
    /// ```
    fn underline<'a>(&'a self, style: Option<&str>) -> ColorPrint<'a>;
}

impl<'a> ColorPrintExt for &'a str {
    fn color<'b>(&'b self, r: u8, g: u8, b: u8) -> ColorPrint<'b> {
        ColorPrint(self, Some((r, g, b)), UnderlineStyle::None)
    }

    fn underline<'b>(&'b self, style: Option<&str>) -> ColorPrint<'b> {
        let underline_style = match style {
            Some("stripe") => UnderlineStyle::String("stripe".to_string()),
            _ => UnderlineStyle::None,
        };
        ColorPrint(self, None, underline_style)
    }
}

impl<'a> ColorPrintExt for ColorPrint<'a> {
    fn color<'b>(&'b self, r: u8, g: u8, b: u8) -> ColorPrint<'b> {
        ColorPrint(self.0, Some((r, g, b)), self.2.clone())
    }

    fn underline<'b>(&'b self, style: Option<&str>) -> ColorPrint<'b> {
        let underline_style = match style {
            Some("upper") => UnderlineStyle::String("upper".to_string()),
            Some("stripe") => UnderlineStyle::String("stripe".to_string()),
            _ => UnderlineStyle::None,
        };
        ColorPrint(self.0, self.1, underline_style)
    }
}

// Test usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_underline() {
        println!("{}", "Hello, world!".color(255, 0, 0).underline(None)); // Con color y subrayado
        println!("{}", "Hello, world!".color(0, 0, 255).underline(Some("stripe"))); // Con color y subrayado superior
    }
}
