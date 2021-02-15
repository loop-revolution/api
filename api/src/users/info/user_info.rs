use super::{
	password::verify_pwd,
	username::{localize_username, validate_username},
};
use crate::{graphql::ContextData, users::user::UserObject};
use async_graphql::*;
use block_tools::{
	auth::{require_token, validate_token},
	models::User,
	UserError,
};

const USERNAME_UPDATE_COST: i32 = 50;

#[derive(Default)]
pub struct UserInfoMutations {}

#[Object]
impl UserInfoMutations {
	/// Update a user's username. This costs 50 credits, and the user must supply their password
	/// and have their token. The new username must not be in use (or it will error).
	async fn update_username(
		&self,
		context: &Context<'_>,
		new_username: String,
		password: String,
	) -> Result<UserObject> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;
		let user = match User::by_id(user_id, conn)? {
			None => return Err(UserError::JwtGeneric.into()),
			Some(user) => user,
		};

		// Make sure the password match
		if !verify_pwd(&password, &user.password)? {
			return Err(UserError::PasswordMatch.into());
		}

		let new_balance = user.credits - USERNAME_UPDATE_COST;
		if new_balance < 0 {
			return Err(UserError::InsufficientFunds(USERNAME_UPDATE_COST).into());
		}

		let new_local = localize_username(&new_username);
		validate_username(&new_local, conn)?;

		Ok(user
			.update_username(&new_username, &new_local, new_balance, conn)?
			.into())
	}

	/// Update a user's display name. This is free and does not require more
	/// than a correct token.
	async fn update_display_name(
		&self,
		context: &Context<'_>,
		new_display_name: String,
	) -> Result<UserObject> {
		let (context, conn) = &ContextData::parse(context)?;

		let user_id = validate_token(&require_token(context)?)?;
		let user = match User::by_id(user_id, conn)? {
			None => return Err(UserError::JwtGeneric.into()),
			Some(user) => user,
		};

		Ok(user.update_display_name(&new_display_name, conn)?.into())
	}
}
