use crossterm::style::{Attribute, Attributes, Color, ContentStyle};
use once_cell::sync::Lazy;

/// Colours for UI output on terminal
#[derive(Debug)]
pub struct Colours;

impl Colours {
    /// Logo left color.
    pub const LOGO_LEFT: Lazy<ContentStyle> = Lazy::new(|| ContentStyle {
        foreground_color: Some(Color::Blue),
        background_color: None,
        attributes: Attributes::from(Attribute::Bold),
    });
    /// Logo left color.
    pub const LOGO_RIGHT: Lazy<ContentStyle> = Lazy::new(|| ContentStyle {
        foreground_color: Some(Color::Grey),
        background_color: None,
        attributes: Attributes::from(Attribute::Bold),
    });
}
