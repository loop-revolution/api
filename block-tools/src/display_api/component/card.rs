use super::{
	button::ButtonComponent, icon::Icon, menu::MenuComponent, text::TextComponent, DisplayComponent,
};
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct CardComponent {
	pub color: Option<String>,
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
	pub buttons: Option<Vec<ButtonComponent>>,
	pub menu: Option<MenuComponent>,
}

impl CardHeader {
	pub fn new(title: &str) -> CardHeader {
		CardHeader {
			title: title.to_string(),
			icon: None,
			block_id: None,
			menu: None,
			buttons: None,
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

	pub fn menu(self, menu: MenuComponent) -> CardHeader {
		CardHeader {
			menu: Some(menu),
			..self
		}
	}

	pub fn button(self, button: ButtonComponent) -> CardHeader {
		let mut buttons = match self.buttons {
			None => vec![],
			Some(buttons) => buttons,
		};
		buttons.push(button);
		CardHeader {
			buttons: Some(buttons),
			..self
		}
	}
}

pub fn error_card(error: &str) -> CardComponent {
	CardComponent {
		color: None,
		content: Box::new(TextComponent::new(error).color("#ff0000")),
		header: CardHeader::new("Block Error"),
	}
}
