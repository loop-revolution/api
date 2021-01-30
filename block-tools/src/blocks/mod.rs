use crate::{
	display_api::{component::DisplayComponent, CreationObject, DisplayObject},
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

#[async_trait]
pub trait BlockType {
	fn name(&self) -> &str;
	async fn create(input: String, context: &Context, user_id: i32) -> Result<Block, Error>;
	async fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, Error>;
	async fn embed_display(
		block: &Block,
		context: &Context,
	) -> Result<Box<dyn DisplayComponent>, Error>;
	async fn create_display(context: &Context, user_id: i32) -> Result<CreationObject, Error>;
}
