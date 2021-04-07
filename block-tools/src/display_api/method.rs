use super::component::DisplayComponent;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MethodObject {
	#[serde(rename = "type")]
	pub block_type: String,
	pub block_id: String,
	pub method_name: String,
	pub arg_template: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreationObject {
	pub header_component: DisplayComponent,
	pub main_component: DisplayComponent,
	pub input_template: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WrappedMethod {
	pub method: MethodObject,
}

impl From<MethodObject> for WrappedMethod {
	fn from(object: MethodObject) -> Self {
		WrappedMethod { method: object }
	}
}

