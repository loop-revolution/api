use erased_serde::Serialize as Serializable;
use serde::{
	ser::{SerializeStruct, Serializer},
	Serialize,
};
use std::fmt;
pub mod button;
pub mod card;
pub mod icon;
pub mod input;
pub mod menu;
pub mod search;
pub mod stack;
pub mod text;

pub trait DisplayComponent {
	fn cid(&self) -> &str;
	fn args(&self) -> &dyn Serializable;
}

impl Serialize for Box<dyn DisplayComponent> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("Component", 2)?;
		state.serialize_field("cid", &self.cid())?;
		state.serialize_field("args", &self.args())?;
		state.end()
	}
}

impl fmt::Debug for dyn DisplayComponent {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Point").field("cid", &self.cid()).finish()
	}
}

#[derive(Serialize)]
pub struct WrappedComponent {
	pub component: Box<dyn DisplayComponent>,
}

impl WrappedComponent {
	pub fn from(component: Box<dyn DisplayComponent>) -> Self {
		WrappedComponent { component }
	}
}
