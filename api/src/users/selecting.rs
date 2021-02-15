use crate::graphql::ContextData;
use async_graphql::*;
use block_tools::{
	auth::{require_token, validate_token},
	dsl::prelude::*,
	models::User,
	schema::users,
};

use super::{info::username::localize_username, user::UserObject};

#[derive(Default)]
pub struct UserSelectingQueries;

#[Object]
impl UserSelectingQueries {
	/// Tries to find a user with a matching ID. Will be null if a user is not found.
	async fn user_by_id(&self, context: &Context<'_>, id: i32) -> Result<Option<UserObject>> {
		let (_, conn) = &ContextData::parse(context)?;
		Ok(User::by_id(id, conn)?.map(|user| user.into()))
	}

	/// Tries to find a user with a matching localized username. Will be null if a user is not found.
	async fn user_by_name(
		&self,
		context: &Context<'_>,
		username: String,
	) -> Result<Option<UserObject>> {
		let (_, conn) = &ContextData::parse(context)?;

		let localname = localize_username(&username);

		let user: Option<User> = users::dsl::users
			.filter(users::localuname.eq(localname))
			.limit(1)
			.get_result(conn)
			.optional()?;

		Ok(user.map(|user| user.into()))
	}

	/// The user that reflects the authorization token
	async fn whoami(&self, context: &Context<'_>) -> Result<Option<UserObject>> {
		let (context, conn) = &ContextData::parse(context)?;
		let user_id = validate_token(&require_token(context)?)?;

		Ok(User::by_id(user_id, conn)?.map(|user| user.into()))
	}
}
