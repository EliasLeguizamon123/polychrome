use polychrome::{colors, StyleExt, StyledText, UnderlineStyle, ProgressBar, utils};

#[test]
fn test_basic_styling() {
    println!("Testing basic styling...");
    let styled = "Hello, world!\n".color(255, 0, 0);
    println!("{}", styled);
    
    // Verificar que el string contiene códigos ANSI
    let output = format!("{}", styled);
    assert!(output.contains("\x1b[38;2;255;0;0m"));
    assert!(output.contains("Hello, world!"));
    assert!(output.contains("\x1b[0m"));
}

#[test]
fn test_chained_styling() {
    println!("Testing chained styling...");
    let styled = "Hello, World!\n"
        .styled()
        .color(255, 0, 0)
        .bold()
        .underline(UnderlineStyle::Normal);

    print!("{}", styled);

    let output = format!("{}", styled);
    assert!(output.contains("\x1b[38;2;255;0;0m")); // Red color
    assert!(output.contains("\x1b[1m"));            // Bold
    assert!(output.contains("\x1b[4m"));            // Underline
}

#[test]
fn test_invalid_hex_color() {
    let result = "Hello".hex_color("Invalid");
    assert!(result.is_err());

    let result = "Hello".hex_color("#FF");
    assert!(result.is_err());
}

#[test]
fn test_polychrome_text() {
    let result = "Hello, World!\n";
    println!("{}", StyledText::polychrome(&result));

    assert!(StyledText::polychrome(&result).contains("\x1b[38;2;"));
}

#[test]
fn test_style_ext_trait() {
    println!("Testing StyleExt trait methods...");
    
    let bold_text = "Bold".bold();
    let italic_text = "Italic".italic();
    let colored_text = "Colored".color(255, 0, 0);
    
    println!("{}", bold_text);
    println!("{}", italic_text);
    println!("{}", colored_text);
    
    assert!(format!("{}", bold_text).contains("\x1b[1m"));
    assert!(format!("{}", italic_text).contains("\x1b[3m"));
    assert!(format!("{}", colored_text).contains("\x1b[38;2;255;0;0m"));
}

#[test]
fn test_method_chaining() {
    println!("Testing extensive method chaining...");
    let complex = "Complex styling"
        .styled()
        .color(255, 165, 0)     // Orange
        .bg_color(0, 0, 139)    // Dark blue background
        .bold()
        .italic()
        .underline(UnderlineStyle::Normal);
    
    println!("{}", complex);
    
    let output = format!("{}", complex);
    assert!(output.contains("\x1b[38;2;255;165;0m")); // Orange foreground
    assert!(output.contains("\x1b[48;2;0;0;139m"));   // Dark blue background
    assert!(output.contains("\x1b[1m"));              // Bold
    assert!(output.contains("\x1b[3m"));              // Italic
    assert!(output.contains("\x1b[4m"));              // Underline
}

#[test]
fn test_styled_text_new() {
    let styled = StyledText::new("Test text");
    let output = format!("{}", styled);
    
    // Should just contain the text without any formatting
    assert!(output.contains("Test text"));
}

#[test]
fn test_utility_functions() {
    println!("Testing utility functions...");
    
    // Test color support detection
    let supports_color = utils::supports_color();
    println!("Terminal supports color: {}", supports_color);
    
    // These functions just print escape codes, so we test they don't panic
    utils::reset_formatting();
    utils::show_cursor();
    utils::hide_cursor();
    
    // Test cursor movement (just ensure it doesn't panic)
    utils::move_cursor(1, 1);
}

#[test]
fn test_underline_styles() {
    println!("Testing underline styles...");
    
    let normal = "Normal underline".underline(UnderlineStyle::Normal);
    let strike = "Strikethrough".underline(UnderlineStyle::Strikethrough);
    let double = "Double underline".underline(UnderlineStyle::Double);
    
    println!("{}", normal);
    println!("{}", strike);
    println!("{}", double);
    
    assert!(format!("{}", normal).contains("\x1b[4m"));
    assert!(format!("{}", strike).contains("\x1b[9m"));
    assert!(format!("{}", double).contains("\x1b[21m"));
}

#[test]
fn test_multiple_text_styles() {
    println!("Testing multiple text styles...");
    let styled = "Multi-style text"
        .styled()
        .bold()
        .italic()
        .dim()
        .color(colors::CYAN.0, colors::CYAN.1, colors::CYAN.2);
    println!("{}", styled);
    
    let output = format!("{}", styled);
    assert!(output.contains("\x1b[1m"));  // Bold
    assert!(output.contains("\x1b[3m"));  // Italic
    assert!(output.contains("\x1b[2m"));  // Dim
}

#[test]
fn test_background_hex_color() {
    println!("Testing background hex color...");
    let styled = "Hex background test"
        .styled()
        .hex_color("#FFFFFF").unwrap()
        .bg_hex_color("#FF0000").unwrap();
    println!("{}", styled);
    
    let output = format!("{}", styled);
    assert!(output.contains("\x1b[38;2;255;255;255m")); // Foreground white
    assert!(output.contains("\x1b[48;2;255;0;0m"));     // Background red
}

#[test]
fn test_background_colors() {
    println!("Testing background colors...");
    let styled = "Background test"
        .styled()
        .color(255, 255, 255)
        .bg_color(255, 0, 0);
    println!("{}", styled);
    
    let output = format!("{}", styled);
    assert!(output.contains("\x1b[38;2;255;255;255m")); // Foreground white
    assert!(output.contains("\x1b[48;2;255;0;0m"));     // Background red
}

#[test]
fn test_predefined_colors() {
    println!("Testing predefined colors...");
    println!("{}", "Red".color(colors::RED.0, colors::RED.1, colors::RED.2));
    println!("{}", "Green".color(colors::GREEN.0, colors::GREEN.1, colors::GREEN.2));
    println!("{}", "Blue".color(colors::BLUE.0, colors::BLUE.1, colors::BLUE.2));
    
    // Verify color values
    assert_eq!(colors::RED, (255, 0, 0));
    assert_eq!(colors::GREEN, (0, 255, 0));
    assert_eq!(colors::BLUE, (0, 0, 255));
    assert_eq!(colors::WHITE, (255, 255, 255));
    assert_eq!(colors::BLACK, (0, 0, 0));
}

#[test]
fn test_progress_bar_clamp() {
    let bar = ProgressBar::new(10);
    
    // Test values outside 0.0-1.0 range
    let negative = bar.render(-0.5);
    let over_one = bar.render(1.5);
    
    // Should be clamped to valid range
    assert_eq!(negative.chars().filter(|&c| c == '█').count(), 0);
    assert_eq!(over_one.chars().filter(|&c| c == '░').count(), 0);
}
