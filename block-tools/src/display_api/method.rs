use super::component::DisplayComponent;
use serde::Serialize;

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
