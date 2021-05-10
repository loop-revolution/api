use crate::display_api::{
	colors::ColorScheme,
	component::{
		atomic::{icon::Icon, text::TextComponent},
		menus::menu::MenuComponent,
		DisplayComponent,
	},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardComponent {
	pub color: Option<String>,
	pub content: Box<DisplayComponent>,
	pub header: Option<CardHeader>,
	pub detached_menu: Option<DetachedMenu>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardHeader {
	pub title: String,
	pub icon: Option<Icon>,
	pub block_id: Option<String>,
	pub menu: Option<MenuComponent>,
	pub custom: Option<Box<DisplayComponent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetachedMenu {
	pub menu: MenuComponent,
	pub location: DetachedMenuLocation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DetachedMenuLocation {
	BottomRight,
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
	pub fn new(content: impl Into<DisplayComponent>) -> Self {
		Self {
			color: None,
			content: box content.into(),
			header: None,
			detached_menu: None,
		}
	}
}

impl DetachedMenu {
	pub fn bottom_right(menu: MenuComponent) -> Self {
		Self {
			menu,
			location: DetachedMenuLocation::BottomRight,
		}
	}
}

impl CardComponent {
	pub fn error_card(error: impl ToString) -> Self {
		Self {
			header: Some(CardHeader::new("Block Error")),
			..Self::new(TextComponent {
				color_scheme: Some(ColorScheme::Red),
				..TextComponent::new(error)
			})
		}
	}
}

impl From<CardComponent> for DisplayComponent {
	fn from(c: CardComponent) -> Self {
		Self::Card(c)
	}
}
