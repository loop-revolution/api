use crate::{
	display_api::{
		component::{card::Icon, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::Block,
	Error, PostgresPool,
};
use async_trait::async_trait;

/// The context to share among GraphQL requests
pub struct Context {
	/// Gives the GraphQL operations access to the DB
	pub pool: PostgresPool,
	pub auth_token: Option<String>,
}

pub struct TypeInfo {
	pub name: String,
	pub desc: String,
	pub icon: Icon,
}

#[async_trait]
pub trait BlockType {
	fn name() -> String;
	fn create(input: String, context: &Context, user_id: i32) -> Result<Block, Error>;
	fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error>;
	fn embed_display(block: &Block, context: &Context) -> Box<dyn DisplayComponent>;
	fn create_display(context: &Context, user_id: i32) -> Result<CreationObject, Error>;
	fn method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, Error>;
	fn info() -> TypeInfo;
	fn block_name(block: &Block, context: &Context) -> Result<String, Error>;
}
