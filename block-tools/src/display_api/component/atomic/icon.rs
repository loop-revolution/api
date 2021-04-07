use crate::display_api::colors::ColorScheme;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IconComponent {
	pub icon: Icon,
	pub color: Option<String>,
	pub color_scheme: Option<ColorScheme>,
	pub size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Icon {
	Box,
	Feed,
	Folder,
	Message,
	Plus,
	TaskComplete,
	ThumbsDown,
	ThumbsUp,
	Type,
}

impl fmt::Display for Icon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl From<Icon> for IconComponent {
	fn from(icon: Icon) -> Self {
		Self {
			icon,
			color: None,
			color_scheme: None,
			size: None,
		}
	}
}
