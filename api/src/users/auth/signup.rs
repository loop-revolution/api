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
