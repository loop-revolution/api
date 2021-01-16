use super::{
	auth_payload::AuthPayload,
	email::{make_mailer, verification_code_email},
};
use crate::{
	graphql::{models::EmailConfirm, Context},
	rand_string,
	user_logic::{hash_pwd, localize_username, validate_pwd, verify_username},
	Error,
};
use db::{
	dsl,
	dsl::prelude::*,
	models::{NewPotentialUser, NewUser, PotentialUser, UserD},
	schema::{potential_users, users},
	PgConnect,
};
use errors::{EmailConfirmError, InternalError};
use lettre::Transport;
use rand::{thread_rng, Rng};
use std::time::SystemTime;

pub async fn signup(
	context: &Context,
	username: String,
	password: String,
	email: String,
) -> Result<EmailConfirm, Error> {
	validate_pwd(&password)?;
	// Hash the password
	let hash = hash_pwd(password)?;

	let localuname = &localize_username(&username);
	let mut rng = thread_rng();
	// Code to send to email in order to verify the email
	let verification_code = rng.gen_range(100000..=999999).to_string();
	// Session code for user signing up
	let session_code = rand_string(24);

	// Model the potential user
	let new_potential = NewPotentialUser {
		username: &username,
		password: &hash,
		email: &email,
		verification_code: &verification_code,
		session_code: &session_code,
		created_at: std::time::SystemTime::now(),
	};

	let conn = &context.pool.get()?;
	// Make sure that the username doesn't already exist
	verify_username(localuname, conn)?;

	// Delete that username's request if it exists
	delete_potential_user(&username, conn)?;

	let mailer = make_mailer();
	mailer
		.send(&verification_code_email(
			&email,
			&username,
			&verification_code,
		))
		.map_err(|_| InternalError::EmailError)?;

	// Preform the insertion to the DB
	let potential_user: PotentialUser = dsl::insert_into(potential_users::table)
		.values(&new_potential)
		.get_result(conn)?;

	Ok(EmailConfirm {
		email: potential_user.email,
		session_code: potential_user.session_code,
	})
}

pub async fn confirm_email(
	context: &Context,
	username: String,
	session_code: String,
	verification_code: String,
) -> Result<AuthPayload, Error> {
	let localuname = &localize_username(&username);

	let conn = &context.pool.get()?;
	verify_username(localuname, conn)?;

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
		.as_secs()
		> 300
	{
		delete_potential_user(&username, conn)?;
		return Err(EmailConfirmError::Expired.into());
	}

	// Check that the session codes and verification codes work
	if potential.session_code != session_code || potential.verification_code != verification_code {
		return Err(EmailConfirmError::Invalid.into());
	}

	// The email is confirmed! Add the user to the DB
	let new_user = NewUser {
		username: &potential.username,
		localuname,
		password: &potential.password,
		email: &potential.email,
		credits: 0,
	};

	let new_user: UserD = dsl::insert_into(users::table)
		.values(&new_user)
		.get_result(conn)?;

	// User is created, not potential anymore
	delete_potential_user(&username, conn)?;

	Ok(AuthPayload::new(new_user.id))
}

pub fn delete_potential_user(username: &str, conn: &PgConnect) -> Result<(), Error> {
	dsl::delete(
		potential_users::dsl::potential_users.filter(potential_users::username.eq(username)),
	)
	.execute(conn)?;
	Ok(())
}

#[cfg(test)]
mod test {
	use crate::test::{easy_exec, gen_exec};

	#[tokio::test]
	async fn password_too_short() {
		let (context, schema) = gen_exec(None);

		// Try to signup with a short password
		let query = "mutation { signup (
					username: \"name\",
					password: \"pwd\",
					email: \"dumb\",
				) { sessionCode } }";
		let (_, errors) = easy_exec(&query, (&schema, &context)).await;

		assert!(&errors[0].error().message().contains("[ups]"));
	}
}
