use super::{icon::Icon, DisplayComponent};
use crate::display_api::{colors::ColorScheme, ActionObject};
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
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

impl DisplayComponent for DropdownComponent {
	fn cid(&self) -> &str {
		"dropdown"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
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

#[derive(Serialize, Debug)]
pub struct DropdownOption {
	icon: Option<Icon>,
	text: String,
}

#[derive(Serialize, Debug)]
pub enum DropdownVariant {
	Filled,
	Outline,
}
