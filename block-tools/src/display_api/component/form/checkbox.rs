use crate::display_api::{ActionObject, colors::ColorScheme, component::atomic::text::TextComponent};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CheckboxVariant {
	Default,
	Cancel,
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
