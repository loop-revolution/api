use crate::display_api::ActionObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchComponent {
	pub cid: String,
	pub name: Option<String>,
	#[serde(rename = "type")]
	pub search_type: Option<SearchType>,
	pub then: Option<ActionObject>,
	pub cancel: Option<ActionObject>,
	pub action_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SearchType {
	Block,
	User,
}

impl Default for SearchComponent {
	fn default() -> Self {
		SearchComponent {
			cid: "search".into(),
			name: None,
			search_type: None,
			then: None,
			cancel: None,
			action_text: None,
		}
	}
}
