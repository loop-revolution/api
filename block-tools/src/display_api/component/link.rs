use super::{text::TextComponent, DisplayComponent};
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct LinkComponent {
	pub text: TextComponent,
	pub external: Option<bool>,
	pub app_path: Option<String>,
	pub url: Option<String>,
}

impl DisplayComponent for LinkComponent {
	fn cid(&self) -> &str {
		"link"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl LinkComponent {
	pub fn new(text: TextComponent) -> Self {
		LinkComponent {
			text,
			external: None,
			app_path: None,
			url: None,
		}
	}
}
