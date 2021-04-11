use crate::display_api::{
	component::{atomic::text::TextComponent, DisplayComponent},
	WrappedMethod,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputComponent {
	pub initial_value: Option<String>,
	pub label: Option<String>,
	pub name: Option<String>,
	#[serde(rename = "type")]
	pub input_type: Option<InputType>,
	pub confirm_cancel: Option<ConfirmCancelOptions>,
	pub mask: Option<TextComponent>,
	pub size: Option<InputSize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmCancelOptions {
	pub enabled: bool,
	pub on_confirm: WrappedMethod,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputType {
	Text,
	Number,
	Date,
	Time,
	Frequency,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputSize {
	Large,
	Medium,
	Small,
	MultiLine,
	Flexible,
}

impl Default for InputComponent {
	fn default() -> Self {
		Self {
			initial_value: None,
			label: None,
			name: None,
			input_type: None,
			confirm_cancel: None,
			mask: None,
			size: None,
		}
	}
}

impl InputComponent {
	pub fn with_confirm(&mut self, on_confirm: WrappedMethod) {
		self.confirm_cancel = Some(ConfirmCancelOptions {
			enabled: true,
			on_confirm,
		});
	}
}

impl From<InputComponent> for DisplayComponent {
	fn from(c: InputComponent) -> Self {
		Self::Input(c)
	}
}
