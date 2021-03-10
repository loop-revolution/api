use super::DisplayComponent;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProgressComponent {
	pub value: i32,
	pub max: Option<i32>,
	pub inner_label: Option<String>,
	pub thickness: Option<String>,
	pub color: Option<String>,
}

impl DisplayComponent for ProgressComponent {
	fn cid(&self) -> &str {
		"progress"
	}

	fn args(&self) -> &dyn Serializable {
		self
	}
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
