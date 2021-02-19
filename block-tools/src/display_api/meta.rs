use serde::Serialize;

use super::component::menu::MenuComponent;

#[derive(Serialize, Debug)]
pub struct DisplayMeta {
	pub page: Option<PageMeta>,
}

impl Default for DisplayMeta {
	fn default() -> Self {
		DisplayMeta { page: None }
	}
}

impl DisplayMeta {
	pub fn page(self, page: PageMeta) -> Self {
		DisplayMeta { page: Some(page) }
	}
}

#[derive(Serialize, Debug)]
pub struct PageMeta {
	pub title: Option<String>,
	pub header: Option<String>,
	pub menu: Option<MenuComponent>,
}

impl Default for PageMeta {
	fn default() -> Self {
		Self::new()
	}
}

impl PageMeta {
	pub fn new() -> Self {
		PageMeta {
			title: None,
			header: None,
			menu: None,
		}
	}

	pub fn title(self, title: &str) -> Self {
		PageMeta {
			title: Some(title.to_string()),
			..self
		}
	}

	pub fn header(self, header: &str) -> Self {
		PageMeta {
			header: Some(header.to_string()),
			..self
		}
	}

	pub fn menu(self, menu: MenuComponent) -> Self {
		PageMeta {
			menu: Some(menu),
			..self
		}
	}
}
