use super::block::BlockObject;
use crate::graphql::ContextData;
use async_graphql::{Context, Error, Object};
use block_tools::{
	auth::{
		permissions::{has_perm_level, PermLevel},
		require_token, validate_token,
	},
	models::Block,
	NoAccessSubject, UserError,
};

#[derive(Default)]
pub struct BlockColorMutations;

#[Object]
impl BlockColorMutations {
	/// Changes a block's color to the RGB string provided. The user must have edit access to the block.
	pub async fn update_color(
		&self,
		context: &Context<'_>,
		rgb: String,
		block_id: i64,
	) -> Result<BlockObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let access_err: Error = UserError::NoAccess(NoAccessSubject::EditColor(block_id)).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(block) => block,
			None => return Err(access_err),
		};

		if !has_perm_level(user_id, &block, PermLevel::Edit) {
			return Err(access_err);
		}

		let block = block.update_color(Some(rgb), conn)?;

		Ok(block.into())
	}

	/// Resets a block's color to the default
	pub async fn reset_color(
		&self,
		context: &Context<'_>,
		block_id: i64,
	) -> Result<BlockObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;

		let access_err: Error = UserError::NoAccess(NoAccessSubject::EditColor(block_id)).into();

		let block = match Block::by_id(block_id, conn)? {
			Some(block) => block,
			None => return Err(access_err),
		};

		if !has_perm_level(user_id, &block, PermLevel::Edit) {
			return Err(access_err);
		}

		let block = block.update_color(None, conn)?;

		Ok(block.into())
	}
}
