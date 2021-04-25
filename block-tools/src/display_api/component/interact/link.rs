use crate::display_api::component::{atomic::text::TextComponent, DisplayComponent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkComponent {
	pub text: TextComponent,
	pub external: Option<bool>,
	pub app_path: Option<String>,
	pub url: Option<String>,
	pub no_style: Option<bool>,
}

impl LinkComponent {
	pub fn new(text: TextComponent) -> Self {
		LinkComponent {
			text,
			external: None,
			app_path: None,
			url: None,
			no_style: None,
		}
	}
}

impl From<LinkComponent> for DisplayComponent {
	fn from(component: LinkComponent) -> Self {
		DisplayComponent::Link(component)
	}
}
