use std::fmt;

use crate::display_api::HexCode;

use super::{menu::MenuComponent, text::TextComponent, DisplayComponent};
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
	pub menu: Option<MenuComponent>,
}

impl CardHeader {
	pub fn new(title: &str) -> CardHeader {
		CardHeader {
			title: title.to_string(),
			icon: None,
			block_id: None,
			menu: None,
		}
	}

	pub fn id(self, id: i64) -> CardHeader {
		CardHeader {
			block_id: Some(id.to_string()),
			..self
		}
	}

	pub fn icon(self, icon: Icon) -> CardHeader {
		CardHeader {
			icon: Some(icon),
			..self
		}
	}
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

pub fn error_card(error: &str) -> CardComponent {
	CardComponent {
		color: None,
		content: Box::new(TextComponent::new(error).color("#ff0000")),
		header: CardHeader::new("Block Error"),
	}
}
