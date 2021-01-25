use super::{
	block::Block,
	delegation::{delegate_create, delegate_creation_display},
};
use crate::{
	graphql::Context,
	user_logic::auth::auth_payload::{require_token, validate_token},
	Error,
};
use block_tools::{dsl::prelude::*, models::BlockD, schema::blocks};

pub async fn block_by_id(context: &Context, id: i32) -> Result<Option<Block>, Error> {
	let conn = &context.pool.get()?;

	let block: Option<BlockD> = blocks::dsl::blocks
		.filter(blocks::id.eq(id))
		.limit(1)
		.get_result(conn)
		.optional()?;

	match block {
		None => Ok(None),
		Some(usr) => Ok(Some(usr.into())),
	}
}

pub async fn create_block(
	context: &Context,
	r#type: String,
	input: String,
) -> Result<Block, Error> {
	let user_id = validate_token(require_token(context)?)?;

	Ok(Block::from(
		delegate_create(r#type.as_str(), input, context, user_id).await?,
	))
}

pub async fn creation_display(context: &Context, r#type: String) -> Result<String, Error> {
	let user_id = validate_token(require_token(context)?)?;

	let display = delegate_creation_display(context, &r#type, user_id).await?;
	Ok(serde_json::to_string(&display)?)
}
