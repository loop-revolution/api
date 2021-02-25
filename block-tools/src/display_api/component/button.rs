use super::{icon::Icon, DisplayComponent};
use crate::display_api::ActionObject;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct ButtonComponent {
	pub color_scheme: Option<String>,
	pub icon: Option<Icon>,
	pub interact: Option<ActionObject>,
	pub size: Option<String>,
	pub text: String,
	pub variant: Option<ButtonVariant>,
}

impl DisplayComponent for ButtonComponent {
	fn cid(&self) -> &str {
		"button"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl ButtonComponent {
	pub fn new(text: &str) -> Self {
		ButtonComponent {
			color_scheme: None,
			icon: None,
			interact: None,
			size: None,
			text: text.into(),
			variant: None,
		}
	}

	pub fn interact(self, action: ActionObject) -> Self {
		ButtonComponent {
			interact: Some(action),
			..self
		}
	}
}

#[derive(Serialize, Debug)]
pub enum ButtonVariant {
	Solid,
	Outline,
	Ghost,
	Link,
	Nostyle,
}
