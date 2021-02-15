use crate::{graphql::ContextData, users::user::UserObject};
use async_graphql::*;
use block_tools::{auth::create_token, models::User};

/// An object that contains authentication information
pub struct AuthPayload {
	pub token: String,
	pub user_id: i32,
}

#[Object]
impl AuthPayload {
	/// The user that is authenticated with the associated token (`token`)
	pub async fn user(&self, context: &Context<'_>) -> Result<Option<UserObject>> {
		let (_, conn) = &ContextData::parse(context)?;
		Ok(User::by_id(self.user_id, conn)?.map(|user| user.into()))
	}
	/// The authentication JWT for the `user`. This is used to authenticate API requests
	/// based on the user when used in the authorization header.
	async fn token(&self) -> String {
		self.token.clone()
	}
}

impl AuthPayload {
	pub fn new(user_id: i32) -> Self {
		AuthPayload {
			user_id,
			token: create_token(user_id),
		}
	}
}
