use std::time::SystemTime;

use super::{
	email::{make_mailer, verification_code_email},
	hash_pwd, localize_username, verify_pwd, verify_username,
};
use crate::{
	db::{
		schema::{potential_users, users},
		PgConnect,
	},
	graphql::{
		models::{EmailConfirm, NewPotentialUser, NewUser, PotentialUser, User},
		Context, EmailConfirmError, InternalError,
	},
	rand_string, Error,
};
use diesel::prelude::*;
use lettre::Transport;
use rand::{thread_rng, Rng};

pub async fn signup(
	context: &Context,
	username: String,
	password: String,
	email: String,
) -> Result<EmailConfirm, Error> {
	verify_pwd(&password)?;
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
	let potential_user: PotentialUser = diesel::insert_into(potential_users::table)
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
) -> Result<User, Error> {
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
		None => {
			return Err(Error::EmailConfirmError(EmailConfirmError::NotFound(
				username,
			)))
		}
		Some(usr) => usr,
	};

	if SystemTime::now()
		.duration_since(potential.created_at)?
		.as_secs()
		> 300
	{
		delete_potential_user(&username, conn)?;
		return Err(Error::EmailConfirmError(EmailConfirmError::Expired));
	}

	// Check that the session codes and verification codes work
	if potential.session_code != session_code || potential.verification_code != verification_code {
		return Err(Error::EmailConfirmError(EmailConfirmError::Invalid));
	}

	// The email is confirmed! Add the user to the DB
	let new_user = NewUser {
		username: &potential.username,
		localuname,
		password: &potential.password,
		email: &potential.email,
	};

	let new_user: User = diesel::insert_into(users::table)
		.values(&new_user)
		.get_result(conn)?;

	// User is created, not potential anymore
	delete_potential_user(&username, conn)?;

	Ok(new_user)
}

pub fn delete_potential_user(username: &str, conn: &PgConnect) -> Result<(), Error> {
	diesel::delete(
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
		let (context, schema) = gen_exec();

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
