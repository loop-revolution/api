use super::auth_payload::AuthPayload;
use crate::{
	graphql::ContextData,
	users::info::username::{localize_username, validate_username},
};
use async_graphql::*;
use block_tools::{
	dsl,
	dsl::prelude::*,
	models::{NewUser, PotentialUser, User},
	schema::{potential_users, users},
	EmailConfirmError, PgConnect,
};
use std::time::SystemTime;

#[derive(Default)]
pub struct ConfirmEmailMutation;

#[Object]
impl ConfirmEmailMutation {
	/// Completes the process of adding a user to the database. This is
	/// the second part of the `signup` mutation, and requires the `sessionCode`
	/// returned from it, as well as the `verificationCode` from the email sent in
	/// `signup`. The username provided must be the same as the one from `signup`.
	pub async fn confirm_email(
		&self,
		context: &Context<'_>,
		username: String,
		session_code: String,
		verification_code: String,
	) -> Result<AuthPayload, Error> {
		let (_, conn) = &ContextData::parse(context)?;

		let localuname = localize_username(&username);
		validate_username(&localuname, conn)?;

		// Find this user in the potential user table
		let potential: Option<PotentialUser> = potential_users::dsl::potential_users
			.filter(potential_users::username.eq(&username))
			.limit(1)
			.get_result(conn)
			.optional()?;

		// Match potential errors
		let potential = match potential {
			None => return Err(EmailConfirmError::NotFound(username).into()),
			Some(usr) => usr,
		};

		if SystemTime::now()
			.duration_since(potential.created_at)?
			.as_secs() > 300
		{
			delete_potential_user(&username, conn)?;
			return Err(EmailConfirmError::Expired.into());
		}

		// Check that the session codes and verification codes work
		if potential.session_code != session_code
			|| potential.verification_code != verification_code
		{
			return Err(EmailConfirmError::Invalid.into());
		}

		// The email is confirmed! Add the user to the DB
		let new_user = NewUser {
			username: potential.username,
			localuname,
			password: potential.password,
			email: potential.email,
			credits: 0,
			display_name: potential.display_name,
		};

		let new_user: User = dsl::insert_into(users::table)
			.values(&new_user)
			.get_result(conn)?;

		// User is created, not potential anymore
		delete_potential_user(&username, conn)?;

		Ok(AuthPayload::new(new_user.id))
	}
}

pub fn delete_potential_user(username: &str, conn: &PgConnect) -> async_graphql::Result<()> {
	dsl::delete(
		potential_users::dsl::potential_users.filter(potential_users::username.eq(username)),
	)
	.execute(conn)?;
	Ok(())
}

#[derive(SimpleObject)]
/// The return of the `signup` mutation. Includes the email (again), and
/// a session code required for `confirmEmail`
pub struct EmailConfirmPayload {
	/// The email that the verification code was sent to
	pub email: String,
	/// The session code required for confirming an email and creating a user.
	/// This is not sent to the user, it is only found from the `signup` mutation.
	pub session_code: String,
}
