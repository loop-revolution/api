use crate::display_api::{
	component::{atomic::icon::Icon, DisplayComponent},
	ActionObject,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionPopoverComponent {
	pub trigger: Option<Box<DisplayComponent>>,
	pub actions: Vec<ActionPopoverAction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionPopoverAction {
	pub icon: Option<Icon>,
	pub text: String,
	pub interact: Option<ActionObject>,
}

impl Default for ActionPopoverComponent {
	fn default() -> Self {
		Self {
			trigger: None,
			actions: vec![],
		}
	}
}

impl ActionPopoverAction {
	pub fn new(text: impl ToString) -> Self {
		Self {
			icon: None,
			text: text.to_string(),
			interact: None,
		}
	}
}

impl From<ActionPopoverComponent> for DisplayComponent {
	fn from(component: ActionPopoverComponent) -> Self {
		DisplayComponent::ActionPopover(component)
	}
}
