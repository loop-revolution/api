use component::DisplayComponent;
use serde::Serialize;
pub mod component;

#[derive(Serialize, Debug)]
pub struct DisplayObject {
	pub display: Box<dyn DisplayComponent>,
	pub meta: Option<DisplayMeta>,
}

#[derive(Serialize, Debug)]
pub struct DisplayMeta {
	pub page: Option<PageMeta>,
}

#[derive(Serialize, Debug)]
pub struct PageMeta {
	pub title: Option<String>,
	pub header: Option<String>,
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

#[derive(Serialize, Debug)]
pub struct CreationObject {
	pub header_component: Box<dyn DisplayComponent>,
	pub main_component: Box<dyn DisplayComponent>,
	pub input_template: String,
}
