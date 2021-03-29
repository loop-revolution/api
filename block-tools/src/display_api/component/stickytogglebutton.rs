use super::{button::ButtonComponent, DisplayComponent};
use crate::display_api::ActionObject;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct StickyToggleButtonComponent {
	pub button: ButtonComponent,
	pub name: Option<String>,
	pub on_change: Option<ActionObject>,
	pub default_value: Option<bool>,
}

impl DisplayComponent for StickyToggleButtonComponent {
	fn cid(&self) -> &str {
		"stickytogglebutton"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}
