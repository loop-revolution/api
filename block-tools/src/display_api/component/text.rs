use super::DisplayComponent;
use crate::display_api::HexCode;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct TextComponent {
	pub text: String,
	pub color: Option<HexCode>,
	pub preset: Option<TextPreset>,
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
	pub fn new(text: &str) -> Self {
		TextComponent {
			text: text.to_string(),
			color: None,
			preset: None,
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

#[derive(Serialize)]
pub enum TextPreset {
	Default,
	Error,
	Heading,
	Info,
	Warn,
}
