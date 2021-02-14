use async_graphql::*;

use super::{localize_username, user::QLUser, verify_pwd, verify_username};
use crate::graphql::ContextData;
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
	async fn update_username(
		&self,
		context: &Context<'_>,
		new_username: String,
		password: String,
	) -> Result<QLUser> {
		let context = &context.data::<ContextData>()?.other();
		let conn = &context.pool.get()?;
		let user_id = validate_token(&require_token(context)?)?;
		let user = User::by_id(user_id, conn)?;
		let user = match user {
			None => return Err(UserError::JWTGeneric.into()),
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
}
