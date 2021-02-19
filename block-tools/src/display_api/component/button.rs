use super::{icon::Icon, DisplayComponent};
use crate::display_api::ActionObject;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct ButtonComponent {
	pub interact: Option<ActionObject>,
	pub text: String,
	pub color_scheme: Option<String>,
	pub variant: Option<ButtonVariant>,
	pub icon: Option<Icon>,
	pub size: Option<String>,
}

impl DisplayComponent for ButtonComponent {
	fn cid(&self) -> &str {
		"button"
	}

	fn args(&self) -> &dyn Serializable {
		self
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
