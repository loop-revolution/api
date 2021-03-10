use super::DisplayComponent;
use crate::display_api::colors::ColorScheme;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct BadgeComponent {
	pub text: String,
	pub variant: Option<BadgeVariant>,
	pub color_scheme: Option<ColorScheme>,
	pub size: Option<String>,
}

impl DisplayComponent for BadgeComponent {
	fn cid(&self) -> &str {
		"badge"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl BadgeComponent {
	pub fn new(text: &str) -> Self {
		BadgeComponent {
			color_scheme: None,
			size: None,
			text: text.into(),
			variant: None,
		}
	}
}

#[derive(Serialize, Debug)]
pub enum BadgeVariant {
	Solid,
	Outline,
	Subtle,
}
