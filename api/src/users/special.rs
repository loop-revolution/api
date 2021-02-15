use async_graphql::*;

use crate::{graphql::ContextData, users::user::UserObject};
use block_tools::{
	auth::{permissions::maybe_use_view, require_token, validate_token},
	models::{Block, User},
	NoAccessSubject, UserError,
};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
/// This is used in mutations to set the type of
/// connection a block has to a user
pub enum SpecialBlock {
	/// The user's central block
	Root,
	/// A block that is featured on the user's profile page
	Featured,
}

#[derive(Default)]
pub struct SpecialBlockMutations {}

#[Object]
impl SpecialBlockMutations {
	/// Updates on of the user's "special" blocks, which
	/// are certain blocks that serve special functions
	async fn set_special_block(
		&self,
		context: &Context<'_>,
		r#type: SpecialBlock,
		block_id: i64,
	) -> Result<UserObject> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;
		let user = match User::by_id(user_id, conn)? {
			None => return Err(UserError::JwtGeneric.into()),
			Some(user) => user,
		};

		let block = Block::by_id(block_id, conn)?;
		// If the block doesn't exist or the user doesn't have access, return an error
		if maybe_use_view(context, block)?.is_none() {
			return Err(UserError::NoAccess(NoAccessSubject::ViewBlock(block_id)).into());
		};

		Ok(match r#type {
			SpecialBlock::Root => user.update_root(Some(block_id), conn)?,
			SpecialBlock::Featured => user.update_featured(Some(block_id), conn)?,
		}
		.into())
	}

	/// Removes the block that serves the "special" role given
	async fn remove_special_block(
		&self,
		context: &Context<'_>,
		r#type: SpecialBlock,
	) -> Result<UserObject> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;
		let user = match User::by_id(user_id, conn)? {
			None => return Err(UserError::JwtGeneric.into()),
			Some(user) => user,
		};

		Ok(match r#type {
			SpecialBlock::Root => user.update_root(None, conn)?,
			SpecialBlock::Featured => user.update_featured(None, conn)?,
		}
		.into())
	}
}
