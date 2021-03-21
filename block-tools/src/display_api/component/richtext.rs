use crate::display_api::MethodObject;

use super::DisplayComponent;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct RichTextComponent {
	/// Should only be a Text or Link component
	pub content: Vec<Box<dyn DisplayComponent>>,
	pub editable: Option<bool>,
	pub name: Option<String>,
	pub save: Option<MethodObject>,
	pub on_enter: Option<MethodObject>,
}

impl DisplayComponent for RichTextComponent {
	fn cid(&self) -> &str {
		"richtext"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl RichTextComponent {
	pub fn new(content: Vec<Box<dyn DisplayComponent>>) -> Self {
		RichTextComponent {
			content,
			editable: None,
			name: None,
			save: None,
			on_enter: None,
		}
	}
}
