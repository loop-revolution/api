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

#[derive(Serialize)]
pub enum TextPreset {
	Heading,
}
