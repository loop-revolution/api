use super::block::BlockObject;
use crate::graphql::ContextData;
use async_graphql::{Context, Error, Object};
use block_tools::{
	auth::{permissions::maybe_use_view, require_token, validate_token},
	dsl::prelude::*,
	models::Block,
	schema::blocks,
	NoAccessSubject, UserError,
};
use block_types::delegation::methods::delegate_method;

#[derive(Default)]
pub struct BasicBlockMutations;

#[Object]
impl BasicBlockMutations {
	/// Executes a specific method created by a block type. Takes a method name, block type,
	/// block id, and arguments for the method. Method name and args determine on the block type
	pub async fn block_method(
		&self,
		context: &Context<'_>,
		#[graphql(desc = "The block type that determines the method")] r#type: String,
		#[graphql(desc = "Arguments to help the method")] args: String,
		#[graphql(desc = "Name of the method on the block type")] method_name: String,
		#[graphql(desc = "ID of the block to act on")] block_id: i64,
	) -> Result<BlockObject, Error> {
		let context = &context.data::<ContextData>()?.other();
		Ok(delegate_method(context, r#type, args, method_name, block_id)?.into())
	}

	/// Deletes a block from the database. Currently does not delete everything,
	/// still has artifacts that may exist in the form of data blocks.
	pub async fn delete_block(&self, context: &Context<'_>, block_id: i64) -> Result<i64, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		// Delete the block (only if the owner is authenticated)
		let rows = block_tools::dsl::delete(
			blocks::dsl::blocks
				.filter(blocks::id.eq(block_id))
				.filter(blocks::owner_id.eq(user_id)),
		)
		.execute(conn)?;

		// If 0 rows are deleted, permissions were not there
		match rows {
			0 => Err(Error::from(UserError::NoAccess(
				NoAccessSubject::DeleteBlock(block_id),
			))),
			_ => Ok(block_id),
		}
	}
}

#[derive(Default)]
pub struct BasicBlockQueries;

#[Object]
impl BasicBlockQueries {
	/// Tries to find a block with a matching ID. Will be null if a block is not found.
	async fn block_by_id(
		&self,
		context: &Context<'_>,
		id: i64,
	) -> Result<Option<BlockObject>, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		// Only return it if you have view access
		Ok(maybe_use_view(context, Block::by_id(id, conn)?)?.map(BlockObject::from))
	}
}
