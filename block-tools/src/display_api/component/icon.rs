use std::fmt;

use crate::display_api::colors::ColorScheme;

use super::DisplayComponent;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct IconComponent {
	pub icon: Icon,
	pub color: Option<String>,
	pub color_scheme: Option<ColorScheme>,
	pub size: Option<String>,
}

impl DisplayComponent for IconComponent {
	fn cid(&self) -> &str {
		"icon"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

#[derive(Serialize, Debug, Clone)]
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

impl From<Icon> for String {
	fn from(icon: Icon) -> Self {
		icon.to_string()
	}
}
