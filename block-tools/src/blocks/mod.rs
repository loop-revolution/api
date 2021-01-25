use crate::{
	display_api::{component::DisplayComponent, CreationObject, DisplayObject},
	models::BlockD,
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
	async fn create(input: String, context: &Context, user_id: i32) -> Result<BlockD, Error>;
	async fn page_display(block: &BlockD, context: &Context) -> Result<DisplayObject, Error>;
	async fn embed_display(
		block: &BlockD,
		context: &Context,
	) -> Result<Box<dyn DisplayComponent>, Error>;
	async fn create_display(context: &Context, user_id: i32) -> Result<CreationObject, Error>;
}
