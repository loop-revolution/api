use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlocklistComponent {
	pub initial_value: Option<Vec<i64>>,
	pub name: Option<String>,
	pub able_to_add_items: Option<bool>,
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
