use super::{menu::MenuComponent, DisplayComponent};
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct DisplayListComponent {
	pub items: Vec<DisplayListItem>,
}

impl DisplayComponent for DisplayListComponent {
	fn cid(&self) -> &str {
		"displaylist"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl Default for DisplayListComponent {
	fn default() -> Self {
		DisplayListComponent {
			items: Vec::default(),
		}
	}
}

#[derive(Serialize)]
pub struct DisplayListItem {
	pub component: Box<dyn DisplayComponent>,
	pub menu: Option<MenuComponent>,
}
