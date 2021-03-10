use crate::display_api::WrappedMethod;

use super::{text::TextComponent, DisplayComponent};
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct InputComponent {
	pub initial_value: Option<String>,
	pub label: Option<String>,
	pub name: Option<String>,
	#[serde(rename = "type")]
	pub input_type: Option<InputType>,
	pub confirm_cancel: Option<ConfirmCancelOptions>,
	pub mask: Option<TextComponent>,
}

impl DisplayComponent for InputComponent {
	fn cid(&self) -> &str {
		"input"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl Default for InputComponent {
	fn default() -> Self {
		Self::new()
	}
}

impl InputComponent {
	pub fn new() -> Self {
		InputComponent {
			initial_value: None,
			label: None,
			name: None,
			input_type: None,
			confirm_cancel: None,
			mask: None,
		}
	}

	pub fn name(self, name: &str) -> Self {
		InputComponent {
			name: Some(name.to_string()),
			..self
		}
	}

	pub fn initial_value(self, value: &str) -> Self {
		InputComponent {
			initial_value: Some(value.to_string()),
			..self
		}
	}

	pub fn input_type(self, input_type: InputType) -> Self {
		InputComponent {
			input_type: Some(input_type),
			..self
		}
	}

	pub fn label(self, label: &str) -> Self {
		InputComponent {
			label: Some(label.to_string()),
			..self
		}
	}

	pub fn mask(self, mask: TextComponent) -> Self {
		InputComponent {
			mask: Some(mask),
			..self
		}
	}

	pub fn with_confirm(self, on_confirm: WrappedMethod) -> Self {
		InputComponent {
			confirm_cancel: Some(ConfirmCancelOptions {
				enabled: true,
				on_confirm,
			}),
			..self
		}
	}
}

#[derive(Serialize)]
pub struct ConfirmCancelOptions {
	pub enabled: bool,
	pub on_confirm: WrappedMethod,
}

#[derive(Serialize)]
pub enum InputType {
	Text,
	Number,
	Date,
	Time,
	Frequency,
}
