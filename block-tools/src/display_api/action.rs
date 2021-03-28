use super::{component::search::SearchComponent, MethodObject};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ActionObject {
	pub method: Option<MethodObject>,
	pub search: Option<Box<SearchComponent>>,
	pub redirect: Option<RedirectObject>,
}

impl ActionObject {
	pub fn method(method: MethodObject) -> Self {
		Self {
			method: Some(method),
			search: None,
			redirect: None,
		}
	}
	pub fn search(search_component: SearchComponent) -> Self {
		Self {
			method: None,
			search: Some(box search_component),
			redirect: None,
		}
	}
	pub fn redirect(redirect: RedirectObject) -> Self {
		Self {
			method: None,
			search: None,
			redirect: Some(redirect),
		}
	}
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

#[derive(Serialize, Clone)]
pub struct RedirectObject {
	pub app_path: Option<String>,
	pub url: Option<String>,
}

impl RedirectObject {
	pub fn app_path(path: String) -> Self {
		Self {
			app_path: Some(path),
			url: None,
		}
	}
	pub fn url(url: String) -> Self {
		Self {
			app_path: None,
			url: Some(url),
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
