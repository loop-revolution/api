use crate::{
	display_api::{
		component::{atomic::icon::Icon, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::Block,
	BlockError, Error, PgConnect, PostgresPool,
};
use async_trait::async_trait;

/// The context to share among GraphQL requests
pub struct Context {
	/// Gives the GraphQL operations access to the DB
	pub pool: PostgresPool,
	pub auth_token: Option<String>,
}

impl Context {
	pub fn conn(&self) -> Result<PgConnect, r2d2::Error> {
		self.pool.get()
	}
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
	fn embed_display(block: &Block, context: &Context) -> DisplayComponent;
	fn create_display(context: &Context, user_id: i32) -> Result<CreationObject, Error>;
	fn method_delegate(
		_context: &Context,
		name: String,
		_block_id: i64,
		_args: String,
	) -> Result<Block, Error> {
		Err(BlockError::MethodExist(name, Self::name()).into())
	}
	fn info() -> TypeInfo;
	fn block_name(block: &Block, context: &Context) -> Result<String, Error>;
	fn visibility_update(_context: &Context, _block_id: i64, _public: bool) -> Result<(), Error> {
		Ok(())
	}
}
