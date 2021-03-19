use super::{
	password::{hash_pwd, validate_pwd, verify_pwd},
	username::{localize_username, validate_username},
};
use crate::{graphql::ContextData, users::user::UserObject};
use async_graphql::*;
use block_tools::{
	auth::{require_token, validate_token},
	models::User,
	UserError,
};

const USERNAME_UPDATE_COST: i32 = 0;

#[derive(Default)]
pub struct UserInfoMutations {}

#[Object]
impl UserInfoMutations {
	/// Update a user's username. This costs 0 credits for now, and the user must supply their password
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

	/// Update a user's password. This requires knowledge of the user's
	/// current password. Resetting a forgotten password is not yet implemented.
	async fn update_password(
		&self,
		context: &Context<'_>,
		new_password: String,
		password: String,
	) -> Result<UserObject> {
		validate_pwd(&new_password)?;
		let hash = hash_pwd(new_password)?;

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

		Ok(user.update_password(&hash, conn)?.into())
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		graphql::build_schema,
		rand_string,
		tests::{build_request, expect_tree_val, test_user},
	};
	use block_tools::{auth::create_token, env_db, get_pool};

	#[tokio::test]
	async fn successful_password_update() {
		let pool = get_pool(&env_db());
		let conn = pool.get().unwrap();

		let (user, password) = test_user(&conn);
		let new_password = rand_string(10);
		let token = create_token(user.id);

		let schema = build_schema();

		let request = build_request(
			format!(
				r#"mutation {{updatePassword (newPassword: "{}", password: "{}") {{ id }} }}"#,
				new_password, password
			),
			pool.clone(),
			Some(token),
		);
		let data = schema.execute(request).await.data;
		let data = expect_tree_val(&data, "updatePassword");
		let id = expect_tree_val(data, "id").to_string();
		assert_eq!(id, user.id.to_string());
	}
}
