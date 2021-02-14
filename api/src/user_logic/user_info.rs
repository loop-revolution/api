use async_graphql::*;

use super::{localize_username, user::UserObject, verify_pwd, verify_username};
use crate::graphql::ContextData;
use block_tools::{
	auth::{permissions::maybe_use_view, require_token, validate_token},
	models::{Block, User},
	NoAccessSubject, UserError,
};

const USERNAME_UPDATE_COST: i32 = 50;

#[derive(Default)]
pub struct UserInfoMutations {}

#[Object]
impl UserInfoMutations {
	async fn update_username(
		&self,
		context: &Context<'_>,
		new_username: String,
		password: String,
	) -> Result<UserObject> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;
		let user = User::by_id(user_id, conn)?;
		let user = match user {
			None => return Err(UserError::JwtGeneric.into()),
			Some(user) => user,
		};
		if !verify_pwd(&password, &user.password)? {
			return Err(UserError::PasswordMatch.into());
		}
		let new_balance = user.credits - USERNAME_UPDATE_COST;
		if new_balance < 0 {
			return Err(UserError::InsufficientFunds(USERNAME_UPDATE_COST).into());
		}
		let new_local = localize_username(&new_username);
		verify_username(&new_local, conn)?;
		Ok(user
			.update_username(&new_username, &new_local, new_balance, conn)?
			.into())
	}

	async fn update_display_name(
		&self,
		context: &Context<'_>,
		new_display_name: String,
	) -> Result<UserObject> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;
		let user = User::by_id(user_id, conn)?;
		let user = match user {
			None => return Err(UserError::JwtGeneric.into()),
			Some(user) => user,
		};
		Ok(user.update_display_name(&new_display_name, conn)?.into())
	}

	async fn set_special_block(
		&self,
		context: &Context<'_>,
		r#type: SpecialBlock,
		block_id: i64,
	) -> Result<UserObject> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;
		let user = User::by_id(user_id, conn)?;
		let user = match user {
			None => return Err(UserError::JwtGeneric.into()),
			Some(user) => user,
		};
		let block = Block::by_id(block_id, conn)?;
		if maybe_use_view(context, block)?.is_none() {
			return Err(UserError::NoAccess(NoAccessSubject::ViewBlock(block_id)).into());
		};

		let user = match r#type {
			SpecialBlock::Root => user.update_root(Some(block_id), conn)?,
			SpecialBlock::Featured => user.update_featured(Some(block_id), conn)?,
		};

		Ok(user.into())
	}

	async fn remove_special_block(
		&self,
		context: &Context<'_>,
		r#type: SpecialBlock,
	) -> Result<UserObject> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;
		let user = User::by_id(user_id, conn)?;
		let user = match user {
			None => return Err(UserError::JwtGeneric.into()),
			Some(user) => user,
		};
		let user = match r#type {
			SpecialBlock::Root => user.update_root(None, conn)?,
			SpecialBlock::Featured => user.update_featured(None, conn)?,
		};

		Ok(user.into())
	}
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SpecialBlock {
	Root,
	Featured,
}
