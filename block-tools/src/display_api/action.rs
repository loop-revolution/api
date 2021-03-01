use super::{component::search::SearchComponent, MethodObject};
use serde::Serialize;

#[derive(Serialize)]
pub struct ActionObject {
	pub method: Option<MethodObject>,
	pub search: Option<Box<SearchComponent>>,
}

impl ActionObject {
	pub fn method(method: MethodObject) -> Self {
		Self {
			method: Some(method),
			search: None,
		}
	}
	pub fn search(search_component: SearchComponent) -> Self {
		Self {
			method: None,
			search: Some(box search_component),
		}
	}
}

impl Default for ActionObject {
	fn default() -> Self {
		Self {
			method: None,
			search: None,
		}
	}
}
