use super::{
	block::BlockObject,
	block_types::{type_list, BlockType},
	delegation::{delegate_create, delegate_creation_display, delegate_method},
};
use crate::graphql::ContextData;
use async_graphql::{Context, Error, Object};
use block_tools::{
	auth::{require_token, validate_token},
	dsl::prelude::*,
	models::Block,
	schema::blocks,
	NoAccessSubject, UserError,
};

#[derive(Default)]
pub struct BlockMutations;

#[Object]
impl BlockMutations {
	/// This mutation attempts to create a block of the block type specified.
	/// An authorization header is required to authenticate the creation, and
	/// a JSON input string should be passed based on the block type.
	pub async fn create_block(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "Name of the block type to create.")] r#type: String,
		#[graphql(desc = "JSON string to specify what to create.")] input: String,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?;
		create_block(context, r#type, input).await
	}

	/// Executes a specific method created by a block type. Takes
	/// method name, block type, block id, and arguments for the
	/// method. Method name and args determine on the block type
	pub async fn block_method(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "The block type that determines the method")] r#type: String,
		#[graphql(desc = "Arguments to help the method")] args: String,
		#[graphql(desc = "Name of the method on the block type")] method_name: String,
		#[graphql(desc = "ID of the block to act on")] block_id: i64,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?;
		Ok(
			delegate_method(context, r#type, args, method_name, block_id)
				.await?
				.into(),
		)
	}

	/// Deletes a block from the database. Currently does not delete everything,
	/// still has artifacts that may exist in the form of data blocks.
	pub async fn delete_block(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "ID of the block to delete.")] block_id: i64,
	) -> Result<i64, Error> {
		let context = &context.data::<ContextData>()?;
		let user_id = validate_token(require_token(&context.other())?)?;
		let conn = &context.pool.get()?;
		match block_tools::dsl::delete(
			blocks::dsl::blocks
				.filter(blocks::id.eq(block_id))
				.filter(blocks::owner_id.eq(user_id)),
		)
		.execute(conn)?
		{
			0 => Err(Error::from(UserError::NoAccess(
				NoAccessSubject::DeleteBlock(block_id),
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
	let user_id = validate_token(require_token(&context.other())?)?;

	Ok(BlockObject::from(
		delegate_create(r#type.as_str(), input, context, user_id).await?,
	))
}

#[derive(Default)]
pub struct BlockQueries;
#[Object]
impl BlockQueries {
	/// Tries to find a block with a matching ID. Will be null if a block is not found.
	async fn block_by_id(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "ID of the block to try to find.")] id: i64,
	) -> Result<Option<BlockObject>, Error> {
		let context = &context.data::<ContextData>()?;
		let conn = &context.pool.get()?;
		Ok(Block::by_id(id, conn)?.and_then(|block| Some(BlockObject::from(block))))
	}

	/// Returns a creation object based on the block type
	/// that is queried. Will have a template string to send
	/// back with the createBlock mutation
	async fn block_creation_display(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "Name of the block type to create.")] r#type: String,
	) -> Result<String, Error> {
		let context = &context.data::<ContextData>()?;
		creation_display(context, r#type).await
	}

	async fn block_types(&self) -> Vec<BlockType> {
		type_list()
	}
}

pub async fn creation_display(context: &ContextData, r#type: String) -> Result<String, Error> {
	let user_id = validate_token(require_token(&context.other())?)?;

	let display = delegate_creation_display(context, &r#type, user_id).await?;
	Ok(serde_json::to_string(&display)?)
}
