use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressComponent {
	pub value: i32,
	pub max: Option<i32>,
	pub inner_label: Option<String>,
	pub thickness: Option<String>,
	pub color: Option<String>,
}

impl ProgressComponent {
	pub fn new(value: i32) -> Self {
		ProgressComponent {
			value,
			max: None,
			inner_label: None,
			thickness: None,
			color: None,
		}
	}
}
