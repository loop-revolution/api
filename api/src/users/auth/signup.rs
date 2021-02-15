use super::{
	confirm_email::delete_potential_user,
	email::{make_mailer, verification_code_email},
};
use crate::{
	graphql::ContextData,
	rand_string,
	users::info::{
		password::{hash_pwd, validate_pwd},
		username::{localize_username, validate_username},
	},
};
use async_graphql::*;
use block_tools::InternalError;
use block_tools::{
	dsl,
	dsl::prelude::*,
	models::{NewPotentialUser, PotentialUser},
	schema::potential_users,
};
use lettre::Transport;
use rand::{thread_rng, Rng};

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
		let hash = hash_pwd(password)?;

		let localuname = localize_username(&username);

		let mut rng = thread_rng();
		let verification_code = rng.gen_range(100000..=999999).to_string();
		let session_code = rand_string(24);

		let new_potential = NewPotentialUser {
			username: &username,
			password: &hash,
			email: &email,
			verification_code: &verification_code,
			session_code: &session_code,
			created_at: std::time::SystemTime::now(),
			display_name: display_name.clone(),
		};

		let (_, conn) = &ContextData::parse(context)?;

		validate_username(&localuname, conn)?;

		// Delete that username's request if it exists
		delete_potential_user(&username, conn)?;

		let mailer = make_mailer();
		mailer
			.send(&verification_code_email(
				&email,
				&display_name.unwrap_or_else(|| username.clone()),
				&verification_code,
			))
			.map_err(|e| {
				block_tools::sentry::capture_error(&e);
				InternalError::EmailError
			})?;

		// Preform the insertion to the DB
		let potential_user: PotentialUser = dsl::insert_into(potential_users::table)
			.values(&new_potential)
			.get_result(conn)?;

		Ok(EmailConfirm {
			email: potential_user.email,
			session_code: potential_user.session_code,
		})
	}
}

#[derive(SimpleObject)]
/// The return of the `signup` mutation. Includes the email (again), and
/// a session code required for `confirmEmail`
pub struct EmailConfirm {
	/// The email that the verification code was sent to
	pub email: String,
	/// The session code required for confirming an email and creating a user.
	/// This is not sent to the user, it is only found from the `signup` mutation.
	pub session_code: String,
}

#[cfg(test)]
mod tests {
	use crate::{
		graphql::build_schema,
		rand_string,
		tests::{build_request, expect_tree_val, rem_first_and_last},
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
		let request = build_request(
			format!(
				r#"mutation {{signup (username: "{}", password: "{}", email: "fake@e.mail") {{ sessionCode }} }}"#,
				username, password
			),
			pool.clone(),
			None,
		);
		let data = schema.execute(request).await.data;
		let data = expect_tree_val(&data, "signup");
		let session_code = expect_tree_val(data, "sessionCode").to_string();
		let session_code = rem_first_and_last(&session_code);
		let conn = pool.get().unwrap();
		let verification_code: String = potential_users::dsl::potential_users
			.filter(potential_users::username.eq(&username))
			.select(potential_users::verification_code)
			.first(&conn)
			.unwrap();
		let request = build_request(
			format!(
				r#"mutation {{ confirmEmail (username: "{}", sessionCode: "{}", verificationCode: "{}") {{ token, user {{ username }} }} }}"#,
				username, session_code, verification_code,
			),
			pool.clone(),
			None,
		);
		let data = schema.execute(request).await;
		let data = data.data;
		let data = expect_tree_val(&data, "confirmEmail");
		let token = expect_tree_val(data, "token").to_string();
		let token = rem_first_and_last(&token);
		validate_token(&token).unwrap();
		let user = expect_tree_val(data, "user");
		let resulting_username = expect_tree_val(user, "username").to_string();
		let resulting_username = rem_first_and_last(&resulting_username);
		assert_eq!(resulting_username, username);
	}
}
