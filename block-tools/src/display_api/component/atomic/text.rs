use crate::display_api::{colors::ColorScheme, component::DisplayComponent};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextComponent {
	pub text: String,
	pub color: Option<String>,
	pub color_scheme: Option<ColorScheme>,
	pub preset: Option<TextPreset>,
	pub bold: Option<bool>,
	pub italic: Option<bool>,
	pub underline: Option<bool>,
	pub strikethrough: Option<bool>,
	pub monospace: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextPreset {
	Default,
	Error,
	Heading,
	Info,
	Warn,
}

impl TextComponent {
	pub fn new(text: impl ToString) -> Self {
		TextComponent {
			text: text.to_string(),
			color: None,
			color_scheme: None,
			preset: None,
			bold: None,
			italic: None,
			underline: None,
			strikethrough: None,
			monospace: None,
		}
	}
}

impl TextComponent {
	pub fn heading(text: impl ToString) -> Self {
		TextComponent {
			preset: Some(TextPreset::Heading),
			..Self::new(text)
		}
	}
	pub fn info(text: impl ToString) -> Self {
		TextComponent {
			preset: Some(TextPreset::Info),
			..Self::new(text)
		}
	}
}

impl From<TextComponent> for DisplayComponent {
    fn from(c: TextComponent) -> Self {
        Self::Text(c)
    }
}
