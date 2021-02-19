use super::DisplayComponent;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct BlocklistComponent {
	pub initial_value: Option<Vec<i64>>,
	pub name: Option<String>,
	pub able_to_add_items: Option<bool>,
}

impl DisplayComponent for BlocklistComponent {
	fn cid(&self) -> &str {
		"blocklist"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
}

impl Default for BlocklistComponent {
	fn default() -> Self {
		BlocklistComponent {
			initial_value: None,
			name: None,
			able_to_add_items: None,
		}
	}
}
