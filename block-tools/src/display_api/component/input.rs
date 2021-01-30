use crate::display_api::WrappedMethod;

use super::DisplayComponent;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct InputComponent {
	pub initial_value: Option<String>,
	pub label: Option<String>,
	pub name: Option<String>,
	#[serde(rename = "type")]
	pub input_type: Option<String>,
	pub confirm_cancel: Option<ConfirmCancelOptions>,
}

impl DisplayComponent for InputComponent {
	fn cid(&self) -> &str {
		"input"
	}

	fn args(&self) -> &dyn Serializable {
		self
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

	pub fn input_type(self, input_type: &str) -> Self {
		InputComponent {
			input_type: Some(input_type.to_string()),
			..self
		}
	}

	pub fn label(self, label: &str) -> Self {
		InputComponent {
			label: Some(label.to_string()),
			..self
		}
	}

	pub fn cc_options(self, options: ConfirmCancelOptions) -> Self {
		InputComponent {
			confirm_cancel: Some(options),
			..self
		}
	}
}

#[derive(Serialize)]
pub struct ConfirmCancelOptions {
	pub enabled: bool,
	pub on_confirm: WrappedMethod,
}
