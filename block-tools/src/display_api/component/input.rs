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

#[derive(Serialize)]
pub struct ConfirmCancelOptions {
	pub enabled: bool,
	pub on_confirm: WrappedMethod,
}
