use crate::display_api::colors::ColorScheme;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadgeComponent {
	pub text: String,
	pub variant: Option<BadgeVariant>,
	pub color_scheme: Option<ColorScheme>,
	pub size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BadgeVariant {
	Solid,
	Outline,
	Subtle,
}

impl BadgeComponent {
	pub fn new(text: impl ToString) -> Self {
		BadgeComponent {
			color_scheme: None,
			size: None,
			text: text.to_string(),
			variant: None,
		}
	}
}
