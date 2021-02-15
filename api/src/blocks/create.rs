use super::block::BlockObject;
use crate::graphql::ContextData;
use async_graphql::{Context, Error, Object};
use block_tools::auth::{require_token, validate_token};
use block_types::delegation::{display::delegate_creation_display, methods::delegate_create};

#[derive(Default)]
pub struct BlockCreationMutation;

#[Object]
impl BlockCreationMutation {
	/// This mutation attempts to create a block of the block type specified.
	/// An authorization header is required to authenticate the creation, and
	/// a JSON input string should be passed based on the block type. The input string template
	/// is generated from the `blockCreationDisplay` query.
	pub async fn create_block(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "Name of the block type to create.")] r#type: String,
		#[graphql(desc = "JSON string to specify what to create.")] input: String,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?.other();
		let user_id = validate_token(&require_token(context)?)?;

		Ok(BlockObject::from(delegate_create(
			r#type.as_str(),
			input,
			context,
			user_id,
		)?))
	}
}

#[derive(Default)]
pub struct BlockCreationQuery;
#[Object]
impl BlockCreationQuery {
	/// Returns a creation object based on the block type that is queried.
	/// Will have a template string to send back with the `createBlock` mutation.
	async fn block_creation_display(
		&self,
		context: &Context<'_>,
		r#type: String,
	) -> Result<String, Error> {
		let context = &context.data::<ContextData>()?.other();
		let user_id = validate_token(&require_token(context)?)?;

		let object = delegate_creation_display(context, &r#type, user_id)?;
		Ok(serde_json::to_string(&object)?)
	}
}
