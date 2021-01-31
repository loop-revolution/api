use crate::{
	graphql::ContextData,
	user_logic::user::{user_by_id, QLUser},
};
use async_graphql::*;
use block_tools::auth::create_token;

pub struct AuthPayload {
	pub token: String,
	pub user_id: i32,
}

#[Object]
impl AuthPayload {
	pub async fn user(&self, context: &Context<'_>) -> Result<Option<QLUser>, Error> {
		let context = context.data::<ContextData>()?;
		Ok(user_by_id(context, self.user_id).await?)
	}
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
