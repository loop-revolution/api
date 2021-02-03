use std::fmt;

use crate::display_api::HexCode;

use super::DisplayComponent;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct CardComponent {
	pub color: Option<HexCode>,
	pub content: Box<dyn DisplayComponent>,
	pub header: CardHeader,
}

impl DisplayComponent for CardComponent {
	fn cid(&self) -> &str {
		"card"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

#[derive(Serialize)]
pub struct CardHeader {
	pub title: String,
	pub icon: Option<Icon>,
	pub block_id: Option<String>,
}

#[derive(Serialize, Debug)]
pub enum Icon {
	Folder,
	TaskComplete,
	Message,
	Box,
	Type,
	Feed,
}

impl fmt::Display for Icon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
