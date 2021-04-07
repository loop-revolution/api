use std::time::SystemTime;

use super::{
	confirm_email::EmailConfirmPayload,
	email::{make_mailer, verification_code_email},
};
use crate::{graphql::ContextData, rand_string, users::user::UserObject};
use async_graphql::*;
use block_tools::{
	auth::{require_token, validate_token},
	dsl,
	dsl::prelude::*,
	models::{EmailConfirm, NewEmailConfirm, User},
	schema::{email_confirm, users},
	EmailConfirmError, InternalError, LoopError,
};
use lettre::Transport;
use rand::thread_rng;
use rand::Rng;

#[derive(Default)]
pub struct ChangeEmailMutation;

#[Object]
impl ChangeEmailMutation {
	pub async fn change_email(
		&self,
		context: &Context<'_>,
		new_email: String,
	) -> Result<EmailConfirmPayload, Error> {
		let (context, conn) = &ContextData::parse(context)?;
		let user_id = validate_token(&require_token(context)?)?;
		let user = if let Some(usr) = User::by_id(user_id, conn)? {
			usr
		} else {
			return Err(LoopError::GenericError.into());
		};

		let mut rng = thread_rng();
		let verification_code = rng.gen_range(100000..=999999).to_string();
		let session_code = rand_string(24);

		let email_confirm = NewEmailConfirm {
			new_email,
			session_code,
			verification_code,
			user_id,
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
	/// After the `confirmEmail` mutation, confirm the change with this mutation. Push the
	/// session code from the `confirmEmail` results, and get the verification code from the email.
	pub async fn confirm_change_email(
		&self,
		context: &Context<'_>,
		session_code: String,
		verification_code: String,
	) -> Result<UserObject, Error> {
		let (context, conn) = &ContextData::parse(context)?;
		let user_id = validate_token(&require_token(context)?)?;
		let user = if let Some(usr) = User::by_id(user_id, conn)? {
			usr
		} else {
			return Err(LoopError::GenericError.into());
		};

		// Find the confirmation object
		let confirm: Option<EmailConfirm> = email_confirm::dsl::email_confirm
			.filter(email_confirm::user_id.eq(&user_id))
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
				email_confirm::dsl::email_confirm.filter(email_confirm::user_id.eq(user_id)),
			)
			.execute(conn)?;
			return Err(EmailConfirmError::Expired.into());
		}

		// Check that the session codes and verification codes are correct
		if confirm.session_code != session_code || confirm.verification_code != verification_code {
			return Err(EmailConfirmError::Invalid.into());
		}

		let user: User = dsl::update(users::dsl::users.filter(users::id.eq(user_id)))
			.set((users::email.eq(confirm.new_email),))
			.get_result(conn)?;
		dsl::delete(email_confirm::dsl::email_confirm.filter(email_confirm::user_id.eq(user_id)))
			.execute(conn)?;

		Ok(user.into())
	}
}
