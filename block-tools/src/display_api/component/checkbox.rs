use super::{text::TextComponent, DisplayComponent};
use crate::display_api::{colors::ColorScheme, ActionObject};
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct CheckboxComponent {
	pub color_scheme: Option<ColorScheme>,
	pub color: Option<String>,
	pub disabled: Option<bool>,
	pub name: Option<String>,
	pub on_change: Option<ActionObject>,
	pub readonly: Option<bool>,
	pub text: Option<TextComponent>,
	pub value: u8,
	pub variant: Option<CheckboxVariant>,
}

impl DisplayComponent for CheckboxComponent {
	fn cid(&self) -> &str {
		"checkbox"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl CheckboxComponent {
	pub fn new(value: u8) -> Self {
		CheckboxComponent {
			color_scheme: None,
			color: None,
			disabled: None,
			name: None,
			on_change: None,
			readonly: None,
			text: None,
			value,
			variant: None,
		}
	}
}

#[derive(Serialize, Debug)]
pub enum CheckboxVariant {
	Default,
	Cancel,
}
