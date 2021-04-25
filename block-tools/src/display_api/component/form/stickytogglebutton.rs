use crate::display_api::{
	component::{interact::button::ButtonComponent, DisplayComponent},
	ActionObject,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickyToggleButtonComponent {
	pub button: ButtonComponent,
	pub name: Option<String>,
	pub on_change: Option<ActionObject>,
	pub default_value: Option<bool>,
}

impl StickyToggleButtonComponent {
	pub fn new(button: ButtonComponent) -> Self {
		Self {
			button,
			name: None,
			on_change: None,
			default_value: None,
		}
	}
}

impl From<StickyToggleButtonComponent> for DisplayComponent {
	fn from(component: StickyToggleButtonComponent) -> Self {
		Self::StickyToggleButton(component)
	}
}
