use std::fmt;

use super::DisplayComponent;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct IconComponent {
	pub icon: Icon,
	pub color: Option<String>,
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

#[derive(Serialize, Debug)]
pub enum Icon {
	Box,
	Feed,
	Folder,
	Message,
	Plus,
	TaskComplete,
	Type,
}

impl fmt::Display for Icon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
