use super::{
	block::BlockObject,
	delegation::{delegate_create, delegate_creation_display},
};
use crate::{
	graphql::ContextData,
	user_logic::auth::auth_payload::{require_token, validate_token},
};
use async_graphql::{Context, Error, Object};
use block_tools::{dsl::prelude::*, models::Block, schema::blocks, NoAccessSubject, UserError};

pub async fn block_by_id(context: &ContextData, id: i64) -> Result<Option<BlockObject>, Error> {
	let conn = &context.pool.get()?;

	let block: Option<Block> = blocks::dsl::blocks
		.filter(blocks::id.eq(id))
		.limit(1)
		.get_result(conn)
		.optional()?;

	match block {
		None => Ok(None),
		Some(usr) => Ok(Some(usr.into())),
	}
}

#[derive(Default)]
pub struct BlockMutations;

#[Object]
impl BlockMutations {
	pub async fn create_block(
		&self,
		context: &Context<'_>,
		r#type: String,
		input: String,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?;
		create_block(context, r#type, input).await
	}

	pub async fn delete_block(&self, context: &Context<'_>, block_id: i64) -> Result<i64, Error> {
		let context = &context.data::<ContextData>()?;
		let user_id = validate_token(require_token(context)?)?;
		let conn = &context.pool.get()?;
		match block_tools::dsl::delete(
			blocks::dsl::blocks
				.filter(blocks::id.eq(block_id))
				.filter(blocks::owner_id.eq(user_id)),
		)
		.execute(conn)?
		{
			0 => Err(Error::from(UserError::NoAccess(
				NoAccessSubject::DeleteBlock,
			))),
			_ => Ok(block_id),
		}
	}
}

pub async fn create_block(
	context: &ContextData,
	r#type: String,
	input: String,
) -> Result<BlockObject, Error> {
	let user_id = validate_token(require_token(context)?)?;

	Ok(BlockObject::from(
		delegate_create(r#type.as_str(), input, context, user_id).await?,
	))
}

#[derive(Default)]
pub struct BlockQueries;
#[Object]
impl BlockQueries {
	async fn create_block(
		&self,
		context: &Context<'_>,
		r#type: String,
		input: String,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?;
		create_block(context, r#type, input).await
	}
	/// Tries to find a block with a matching ID. Will be null if a block is not found.
	async fn block_by_id(
		&self,
		context: &Context<'_>,
		id: i64,
	) -> Result<Option<BlockObject>, Error> {
		let context = &context.data::<ContextData>()?;
		block_by_id(context, id).await
	}

	async fn block_creation_display(
		&self,
		context: &Context<'_>,
		r#type: String,
	) -> Result<String, Error> {
		let context = &context.data::<ContextData>()?;
		creation_display(context, r#type).await
	}
}

pub async fn creation_display(context: &ContextData, r#type: String) -> Result<String, Error> {
	let user_id = validate_token(require_token(context)?)?;

	let display = delegate_creation_display(context, &r#type, user_id).await?;
	Ok(serde_json::to_string(&display)?)
}
