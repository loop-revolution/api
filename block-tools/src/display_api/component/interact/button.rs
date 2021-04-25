use crate::display_api::{
	colors::ColorScheme,
	component::{atomic::icon::Icon, DisplayComponent},
	ActionObject,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonComponent {
	pub color_scheme: Option<ColorScheme>,
	pub icon: Option<Icon>,
	pub interact: Option<ActionObject>,
	pub size: Option<ButtonSize>,
	pub text: String,
	pub variant: Option<ButtonVariant>,
	pub readonly: Option<bool>,
	pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ButtonVariant {
	Solid,
	Outline,
	Ghost,
	Link,
	Nostyle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ButtonSize {
	Small,
	Medium,
	Large,
}

impl ButtonComponent {
	pub fn new(text: impl ToString) -> Self {
		ButtonComponent {
			color_scheme: None,
			icon: None,
			interact: None,
			size: None,
			text: text.to_string(),
			variant: None,
			disabled: None,
			readonly: None,
		}
	}
}

impl From<ButtonComponent> for DisplayComponent {
	fn from(component: ButtonComponent) -> Self {
		DisplayComponent::Button(component)
	}
}
