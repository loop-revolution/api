use serde::Serialize;

use super::component::{menus::menu::MenuComponent, DisplayComponent};

#[derive(Serialize)]
pub struct DisplayMeta {
	pub page: Option<PageMeta>,
	pub color: Option<String>,
}

#[derive(Serialize)]
pub struct PageMeta {
	pub title: Option<String>,
	pub header: Option<String>,
	pub header_component: Option<DisplayComponent>,
	pub menu: Option<MenuComponent>,
}

impl Default for DisplayMeta {
	fn default() -> Self {
		Self {
			page: None,
			color: None,
		}
	}
}

impl Default for PageMeta {
	fn default() -> Self {
		Self {
			title: None,
			header: None,
			header_component: None,
			menu: None,
		}
	}
}
