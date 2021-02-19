use super::auth_payload::AuthPayload;
use crate::{
	graphql::ContextData,
	users::info::{password::verify_pwd, username::localize_username},
};
use async_graphql::*;
use block_tools::{dsl::prelude::*, models::User, schema::users, UserError};

#[derive(Default)]
pub struct LoginMutations;

#[Object]
impl LoginMutations {
	/// If the provided username and password are correct, will return with an
	/// authentication token & user pair for authenticating requests.
	pub async fn login(
		&self,
		context: &Context<'_>,
		username: String,
		password: String,
	) -> Result<AuthPayload> {
		let (_, conn) = &ContextData::parse(context)?;

		let localuname = localize_username(&username);
		let user: Option<User> = users::dsl::users
			.filter(users::localuname.eq(localuname))
			.first(conn)
			.optional()?;

		let user = match user {
			Some(user) => user,
			None => return Err(UserError::NameNonexist(username).into()),
		};

		if !verify_pwd(&password, &user.password)? {
			return Err(UserError::PasswordMatch.into());
		}

		Ok(AuthPayload::new(user.id))
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		graphql::build_schema,
		tests::{build_request, expect_tree_val, rem_first_and_last, test_user},
	};
	use block_tools::{auth::validate_token, env_db, get_pool};

	#[tokio::test]
	async fn successful_login() {
		let pool = get_pool(&env_db());
		let conn = pool.get().unwrap();

		let (user, password) = test_user(&conn);

		let schema = build_schema();

		let request = build_request(
			format!(
				r#"mutation {{login (username: "{}", password: "{}") {{ token, user {{id}} }} }}"#,
				user.username, password
			),
			pool.clone(),
			None,
		);
		let data = schema.execute(request).await.data;
		let data = expect_tree_val(&data, "login");
		let token = expect_tree_val(data, "token").to_string();
		let token = rem_first_and_last(&token);
		validate_token(&token).unwrap();
		let user_dat = expect_tree_val(data, "user");
		let id: i32 = expect_tree_val(user_dat, "id").to_string().parse().unwrap();
		assert_eq!(id, user.id);
	}
}
