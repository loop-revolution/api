use crate::display_api::colors::ColorScheme;

use super::DisplayComponent;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize, Clone)]
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

impl DisplayComponent for TextComponent {
	fn cid(&self) -> &str {
		"text"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl TextComponent {
	pub fn new(text: impl std::fmt::Display) -> Self {
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

	pub fn color(self, color_code: &str) -> Self {
		TextComponent {
			color: Some(color_code.to_string()),
			..self
		}
	}

	pub fn preset(self, preset: TextPreset) -> Self {
		TextComponent {
			preset: Some(preset),
			..self
		}
	}
}

#[derive(Serialize, Clone)]
pub enum TextPreset {
	Default,
	Error,
	Heading,
	Info,
	Warn,
}
