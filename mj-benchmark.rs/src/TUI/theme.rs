use ratatui::style::{Color, Style};

pub const COLOR_TEXT: Color = Color::Rgb(192, 192, 192);   // Silver Grey
pub const COLOR_BORDER: Color = Color::Rgb(192, 192, 192); // Silver Grey
pub const COLOR_WARNING: Color = Color::Rgb(255, 0, 0);    // Warning Red
pub const COLOR_SUCCESS: Color = Color::Rgb(0, 255, 255);  // Cyan
pub const COLOR_BG: Color = Color::Black;

pub fn border_style() -> Style {
    Style::default().fg(COLOR_BORDER)
}

pub fn text_style() -> Style {
    Style::default().fg(COLOR_TEXT)
}

pub fn success_style() -> Style {
    Style::default().fg(COLOR_SUCCESS)
}

pub fn warning_style() -> Style {
    Style::default().fg(COLOR_WARNING)
}