use super::auth_payload::AuthPayload;
use crate::{
	graphql::Context,
	user_logic::{localize_username, verify_pwd},
	Error,
};
use db::{dsl::prelude::*, models::UserD, schema::users};
use errors::UserError;

pub async fn login(
	context: &Context,
	username: String,
	password: String,
) -> Result<AuthPayload, Error> {
	let conn = &context.pool.get()?;
	let localuname = &localize_username(&username);

	let user: Option<UserD> = users::dsl::users
		.filter(users::localuname.eq(localuname))
		.first(conn)
		.optional()?;

	if let None = user {
		return Err(UserError::NameNonexist(username).into());
	}
	let user = user.unwrap();

	let pwd_pass = verify_pwd(&password, &user.password)?;

	if pwd_pass == false {
		return Err(UserError::PasswordMatch.into());
	}

	Ok(AuthPayload::new(user.id))
}
