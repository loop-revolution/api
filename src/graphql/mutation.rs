use super::{
	context::Context,
	models::{EmailConfirm, User},
	user_logic::auth::{confirm_email, signup},
	Error,
};
use juniper::graphql_object;

/// Struct for GraphQL Mutations
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
	/// Verifies the username and password, then sends an email to confirm it.
	/// Returns a session code that must be
	/// sent along with the verification code using `confirmEmail`, which will then create the user.
	async fn signup(
		context: &Context,
		username: String,
		password: String,
		email: String,
	) -> Result<EmailConfirm, Error> {
		signup(context, username, password, email).await
	}

	async fn confirm_email(
		context: &Context,
		username: String,
		session_code: String,
		verification_code: String,
	) -> Result<User, Error> {
		confirm_email(context, username, session_code, verification_code).await
	}
}
