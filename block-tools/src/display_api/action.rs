use super::MethodObject;
use erased_serde::Serialize as Serializable;
use serde::Serialize;

#[derive(Serialize)]
pub struct ActionObject {
	pub method: Option<MethodObject>,
	pub launch: Option<Box<dyn Serializable>>,
}
