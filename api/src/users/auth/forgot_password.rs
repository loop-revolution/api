use std::time::SystemTime;

use super::{
	auth_payload::AuthPayload,
	confirm_email::EmailConfirmPayload,
	email::{make_mailer, verification_code_email},
};
use crate::{
	graphql::ContextData,
	rand_string,
	users::info::{
		password::{hash_pwd, validate_pwd},
		username::localize_username,
	},
};
use async_graphql::*;
use block_tools::{
	dsl,
	dsl::prelude::*,
	models::{EmailConfirm, NewEmailConfirm, User},
	schema::{email_confirm, users},
	EmailConfirmError, InternalError, UserError,
};
use lettre::Transport;
use rand::thread_rng;
use rand::Rng;

#[derive(Default)]
pub struct ForgotPasswordMutations;

#[Object]
impl ForgotPasswordMutations {
	/// If a user forgets their password, send this request along with their username.
	/// If they have an account, a verification code will be sent to their email, which should be
	/// used for `confirmForgotPassword`.
	pub async fn forgot_password(
		&self,
		context: &Context<'_>,
		username: String,
	) -> Result<EmailConfirmPayload, Error> {
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

		let mut rng = thread_rng();
		let verification_code = rng.gen_range(100000..=999999).to_string();
		let session_code = rand_string(24);

		let email_confirm = NewEmailConfirm {
			new_email: user.email,
			session_code,
			verification_code,
			user_id: user.id,
			created_at: SystemTime::now(),
		};

		// Preform the insertion to the DB
		let email_confirm: EmailConfirm = dsl::insert_into(email_confirm::table)
			.values(&email_confirm)
			.get_result(conn)?;

		let name = user.display_name.clone().unwrap_or(user.username);

		let mailer = make_mailer();
		mailer
			.send(&verification_code_email(
				&email_confirm.new_email,
				&name,
				&email_confirm.verification_code,
			))
			.map_err(|e| {
				block_tools::sentry::capture_error(&e);
				InternalError::EmailError
			})?;

		Ok(EmailConfirmPayload {
			email: email_confirm.new_email,
			session_code: email_confirm.session_code,
		})
	}
	/// After calling `forgotPassword`, this will set the user's password to the password provided.
	/// This will also create a token to log in the user.
	pub async fn confirm_forgot_password(
		&self,
		context: &Context<'_>,
		username: String,
		session_code: String,
		verification_code: String,
		new_password: String,
	) -> Result<AuthPayload, Error> {
		validate_pwd(&new_password)?;
		let hash = hash_pwd(new_password)?;
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

		// Find the confirmation object
		let confirm: Option<EmailConfirm> = email_confirm::dsl::email_confirm
			.filter(email_confirm::user_id.eq(&user.id))
			.first(conn)
			.optional()?;

		// Match potential errors
		let confirm = match confirm {
			None => return Err(EmailConfirmError::NotFound(user.username).into()),
			Some(confirm) => confirm,
		};

		if SystemTime::now()
			.duration_since(confirm.created_at)?
			.as_secs() > 300
		{
			dsl::delete(
				email_confirm::dsl::email_confirm.filter(email_confirm::user_id.eq(user.id)),
			)
			.execute(conn)?;
			return Err(EmailConfirmError::Expired.into());
		}

		// Check that the session codes and verification codes are correct
		if confirm.session_code != session_code || confirm.verification_code != verification_code {
			return Err(EmailConfirmError::Invalid.into());
		}

		let user: User = dsl::update(users::dsl::users.filter(users::id.eq(user.id)))
			.set((users::password.eq(&hash),))
			.get_result(conn)?;

		dsl::delete(email_confirm::dsl::email_confirm.filter(email_confirm::user_id.eq(user.id)))
			.execute(conn)?;

		Ok(AuthPayload::new(user.id))
	}
}
