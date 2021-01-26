use super::auth_payload::AuthPayload;
use crate::{
	graphql::ContextData,
	user_logic::{localize_username, verify_pwd},
};
use async_graphql::*;
use block_tools::{dsl::prelude::*, models::UserD, schema::users, UserError};

#[derive(Default)]
pub struct LoginMutations;

#[Object]
impl LoginMutations {
	pub async fn login(
		&self,
		context: &Context<'_>,
		username: String,
		password: String,
	) -> Result<AuthPayload, Error> {
		let conn = &context.data::<ContextData>()?.pool.get()?;
		let localuname = &localize_username(&username);

		let user: Option<UserD> = users::dsl::users
			.filter(users::localuname.eq(localuname))
			.first(conn)
			.optional()?;

		if user.is_none() {
			return Err(UserError::NameNonexist(username).into());
		}
		let user = user.unwrap();

		let pwd_pass = verify_pwd(&password, &user.password)?;

		if !pwd_pass {
			return Err(UserError::PasswordMatch.into());
		}

		Ok(AuthPayload::new(user.id))
	}
}
