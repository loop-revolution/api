use crate::display_api::{
	colors::ColorScheme,
	component::{
		atomic::{icon::Icon, text::TextComponent},
		menu::menu::MenuComponent,
		DisplayComponent,
	},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardComponent {
	pub color: Option<String>,
	pub content: Box<DisplayComponent>,
	pub header: Box<CardHeader>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardHeader {
	pub title: String,
	pub icon: Option<Icon>,
	pub block_id: Option<String>,
	pub menu: Option<MenuComponent>,
	pub custom: Option<Box<DisplayComponent>>,
}

impl CardHeader {
	pub fn new(title: impl ToString) -> CardHeader {
		CardHeader {
			title: title.to_string(),
			icon: None,
			block_id: None,
			menu: None,
			custom: None,
		}
	}
}

impl CardComponent {
	pub fn error_card(error: impl ToString) -> Self {
		Self {
			color: None,
			content: box TextComponent {
				color_scheme: Some(ColorScheme::Red),
				..TextComponent::new(error)
			}
			.into(),
			header: box CardHeader::new("Block Error"),
		}
	}
}

impl From<CardComponent> for DisplayComponent {
    fn from(c: CardComponent) -> Self {
        Self::Card(c)
    }
}
