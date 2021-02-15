use component::DisplayComponent;
use serde::Serialize;
pub mod component;

#[derive(Serialize, Debug)]
pub struct DisplayObject {
	pub display: Box<dyn DisplayComponent>,
	pub meta: Option<DisplayMeta>,
}

impl DisplayObject {
	pub fn new(component: Box<dyn DisplayComponent>) -> Self {
		DisplayObject {
			display: component,
			meta: None,
		}
	}

	pub fn meta(self, meta: DisplayMeta) -> Self {
		DisplayObject {
			meta: Some(meta),
			..self
		}
	}
}

#[derive(Serialize, Debug)]
pub struct DisplayMeta {
	pub page: Option<PageMeta>,
}

impl Default for DisplayMeta {
	fn default() -> Self {
		DisplayMeta { page: None }
	}
}

impl DisplayMeta {
	pub fn page(self, page: PageMeta) -> Self {
		DisplayMeta { page: Some(page) }
	}
}

#[derive(Serialize, Debug)]
pub struct PageMeta {
	pub title: Option<String>,
	pub header: Option<String>,
}

impl Default for PageMeta {
	fn default() -> Self {
		Self::new()
	}
}

impl PageMeta {
	pub fn new() -> Self {
		PageMeta {
			title: None,
			header: None,
		}
	}

	pub fn title(self, title: &str) -> Self {
		PageMeta {
			title: Some(title.to_string()),
			..self
		}
	}

	pub fn header(self, header: &str) -> Self {
		PageMeta {
			header: Some(header.to_string()),
			..self
		}
	}
}

pub type HexCode = String;

#[derive(Serialize, Debug)]
pub struct MethodObject {
	#[serde(rename = "type")]
	pub block_type: String,
	pub block_id: String,
	pub method_name: String,
	pub arg_template: String,
}

#[derive(Serialize)]
pub struct WrappedMethod {
	pub method: MethodObject,
}

impl From<MethodObject> for WrappedMethod {
	fn from(object: MethodObject) -> Self {
		WrappedMethod { method: object }
	}
}

#[derive(Serialize, Debug)]
pub struct CreationObject {
	pub header_component: Box<dyn DisplayComponent>,
	pub main_component: Box<dyn DisplayComponent>,
	pub input_template: String,
}
