use super::{icon::Icon, DisplayComponent};
use crate::display_api::ActionObject;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct ActionPopoverComponent {
	pub trigger: Option<Box<dyn DisplayComponent>>,
	pub actions: Vec<ActionPopoverAction>,
}

impl DisplayComponent for ActionPopoverComponent {
	fn cid(&self) -> &str {
		"actionpopover"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl Default for ActionPopoverComponent {
	fn default() -> Self {
		Self {
			trigger: None,
			actions: vec![],
		}
	}
}

#[derive(Serialize)]
pub struct ActionPopoverAction {
	icon: Option<Icon>,
	text: String,
	interact: Option<ActionObject>,
}
