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
