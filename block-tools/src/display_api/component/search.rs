use serde::Serialize;

use crate::display_api::ActionObject;

#[derive(Serialize)]
pub struct SearchComponent {
	pub cid: String,
	pub name: Option<String>,
	pub r#type: Option<SearchType>,
	pub then: Option<ActionObject>,
	pub cancel: Option<ActionObject>,
}

impl Default for SearchComponent {
	fn default() -> Self {
		SearchComponent {
			cid: "search".into(),
			name: None,
			r#type: None,
			then: None,
			cancel: None,
		}
	}
}

impl SearchComponent {
	pub fn name(self, name: &str) -> Self {
		Self {
			name: Some(name.to_string()),
			..self
		}
	}
	pub fn r#type(self, r#type: SearchType) -> Self {
		Self {
			r#type: Some(r#type),
			..self
		}
	}
	pub fn then(self, action: ActionObject) -> Self {
		Self {
			then: Some(action),
			..self
		}
	}
	pub fn cancel(self, action: ActionObject) -> Self {
		Self {
			cancel: Some(action),
			..self
		}
	}
}

#[derive(Serialize, Debug)]
pub enum SearchType {
	Block,
	User,
}
