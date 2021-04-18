use crate::display_api::component::{menus::menu::MenuComponent, DisplayComponent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayListComponent {
	pub items: Vec<DisplayListItem>,
	pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayListItem {
	pub component: DisplayComponent,
	pub menu: Option<MenuComponent>,
}

impl Default for DisplayListComponent {
	fn default() -> Self {
		DisplayListComponent {
			items: Vec::default(),
			color: None,
		}
	}
}

impl DisplayListItem {
	pub fn new(component: DisplayComponent) -> Self {
		DisplayListItem {
			component,
			menu: None,
		}
	}
}

impl From<DisplayListComponent> for DisplayComponent {
	fn from(list: DisplayListComponent) -> Self {
		DisplayComponent::DisplayList(list)
	}
}
