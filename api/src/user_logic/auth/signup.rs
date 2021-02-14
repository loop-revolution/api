use super::{
	auth_payload::AuthPayload,
	email::{make_mailer, verification_code_email},
};
use crate::{
	graphql::{models::EmailConfirm, ContextData},
	rand_string,
	user_logic::{hash_pwd, localize_username, validate_pwd, verify_username},
};
use async_graphql::*;
use block_tools::{
	dsl,
	dsl::prelude::*,
	models::{NewPotentialUser, NewUser, PotentialUser, User},
	schema::{potential_users, users},
	PgConnect,
};
use block_tools::{EmailConfirmError, InternalError};
use lettre::Transport;
use rand::{thread_rng, Rng};
use std::time::SystemTime;

#[derive(Default)]
pub struct SignupMutations;

#[Object]
impl SignupMutations {
	/// Verifies the username and password, then sends an email to confirm it.
	/// Returns a session code that must be
	/// sent along with the verification code using `confirmEmail`, which will then create the user.
	pub async fn signup(
		&self,
		context: &Context<'_>,
		username: String,
		password: String,
		email: String,
		display_name: Option<String>,
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
			display_name: display_name.clone(),
		};

		let conn = &context.data::<ContextData>()?.pool.get()?;

		// Make sure that the username doesn't already exist
		verify_username(localuname, conn)?;

		// Delete that username's request if it exists
		delete_potential_user(&username, conn)?;

		let mailer = make_mailer();
		mailer
			.send(&verification_code_email(
				&email,
				&display_name.unwrap_or(username.clone()),
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
		&self,
		context: &Context<'_>,
		username: String,
		session_code: String,
		verification_code: String,
	) -> Result<AuthPayload, Error> {
		let localuname = localize_username(&username);

		let conn = &context.data::<ContextData>()?.pool.get()?;
		verify_username(&localuname, conn)?;

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
pub fn delete_potential_user(username: &str, conn: &PgConnect) -> Result<(), Error> {
	dsl::delete(
		potential_users::dsl::potential_users.filter(potential_users::username.eq(username)),
	)
	.execute(conn)?;
	Ok(())
}

#[cfg(test)]
mod test {
	use crate::{
		graphql::build_schema,
		rand_string,
		tests::{make_request, rem_first_and_last, value_of_value},
	};
	use block_tools::{
		auth::validate_token, dsl::prelude::*, env_db, get_pool, schema::potential_users,
	};

	#[tokio::test]
	async fn password_too_short() {
		let schema = build_schema();

		// Try to signup with a short password
		let query = "mutation { signup (
					username: \"name\",
					password: \"pwd\",
					email: \"fake@e.mail\",
				) { sessionCode } }";
		let res = schema.execute(query).await;

		assert!(res.errors[0].message.contains("[ups]"));
	}

	#[tokio::test]
	async fn successful_signup() {
		let pool = get_pool(&env_db());
		let schema = build_schema();
		let username = rand_string(10);
		let password = rand_string(10);
		let request = make_request(
			format!(
				r#"mutation {{signup (username: "{}", password: "{}", email: "fake@e.mail") {{ sessionCode }} }}"#,
				username, password
			),
			pool.clone(),
			None,
		);
		let data = schema.execute(request).await.data;
		let data = value_of_value(&data, "signup");
		let session_code = value_of_value(data, "sessionCode").to_string();
		let session_code = rem_first_and_last(&session_code);
		let conn = pool.get().unwrap();
		let verification_code: String = potential_users::dsl::potential_users
			.filter(potential_users::username.eq(&username))
			.select(potential_users::verification_code)
			.first(&conn)
			.unwrap();
		let request = make_request(
			format!(
				r#"mutation {{ confirmEmail (username: "{}", sessionCode: "{}", verificationCode: "{}") {{ token, user {{ username }} }} }}"#,
				username, session_code, verification_code,
			),
			pool.clone(),
			None,
		);
		let data = schema.execute(request).await;
		let data = data.data;
		let data = value_of_value(&data, "confirmEmail");
		let token = value_of_value(data, "token").to_string();
		let token = rem_first_and_last(&token);
		validate_token(&token).unwrap();
		let user = value_of_value(data, "user");
		let resulting_username = value_of_value(user, "username").to_string();
		let resulting_username = rem_first_and_last(&resulting_username);
		assert_eq!(resulting_username, username);
	}
}
