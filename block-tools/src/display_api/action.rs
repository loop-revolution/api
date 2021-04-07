use super::{MethodObject, component::misc::search::SearchComponent};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionObject {
	pub method: Option<MethodObject>,
	pub search: Option<Box<SearchComponent>>,
	pub redirect: Option<RedirectObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedirectObject {
	pub app_path: Option<String>,
	pub url: Option<String>,
}

impl Default for ActionObject {
	fn default() -> Self {
		Self {
			method: None,
			search: None,
			redirect: None,
		}
	}
}

impl ActionObject {
	pub fn method(method: MethodObject) -> Self {
		Self {
			method: Some(method),
			..Default::default()
		}
	}
	pub fn search(search: SearchComponent) -> Self {
		Self {
			search: Some(box search),
			..Default::default()
		}
	}
	pub fn redirect(redirect: RedirectObject) -> Self {
		Self {
			redirect: Some(redirect),
			..Default::default()
		}
	}
}

impl Default for RedirectObject {
	fn default() -> Self {
		Self {
			app_path: None,
			url: None,
		}
	}
}

impl RedirectObject {
	pub fn app_path(app_path: impl ToString) -> Self {
		Self {
			app_path: Some(app_path.to_string()),
			..Default::default()
		}
	}
	pub fn url(url: impl ToString) -> Self {
		Self {
			url: Some(url.to_string()),
			..Default::default()
		}
	}
}
