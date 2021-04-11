use crate::display_api::{colors::ColorScheme, component::atomic::icon::Icon, ActionObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DropdownComponent {
	pub color_scheme: Option<ColorScheme>,
	pub default: Option<u8>,
	pub disabled: Option<bool>,
	pub name: Option<String>,
	pub on_change: Option<ActionObject>,
	pub options: Vec<DropdownOption>,
	pub readonly: Option<bool>,
	pub variant: Option<DropdownVariant>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DropdownOption {
	icon: Option<Icon>,
	text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DropdownVariant {
	Filled,
	Outline,
}

impl Default for DropdownComponent {
	fn default() -> Self {
		Self {
			color_scheme: None,
			default: None,
			disabled: None,
			name: None,
			on_change: None,
			options: vec![],
			readonly: None,
			variant: None,
		}
	}
}
