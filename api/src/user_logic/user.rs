use super::auth::auth_payload::{require_token, validate_token};
use crate::{block_logic::block::Block, graphql::ContextData};
use async_graphql::*;
use block_tools::{
	dsl::prelude::*,
	models::{BlockD, UserD},
	schema::{blocks, users},
};
pub struct User {
	/// Auto-incrementing unique ID for a user
	pub id: i32,
	/// Unique alphanumeric username for easy identification
	pub username: String,
}

#[Object]
impl User {
	/// How many users there are in the database
	async fn credits(&self, context: &Context<'_>) -> Result<Option<i32>, Error> {
		let context = &context.data::<ContextData>()?;
		let conn = &context.pool.get()?;
		let token = require_token(context)?;

		if self.id != validate_token(token)? {
			return Ok(None);
		}

		Ok(Some(
			users::dsl::users
				.filter(users::id.eq(&self.id))
				.select(users::credits)
				.first(conn)?,
		))
	}

	async fn id(&self) -> i32 {
		self.id
	}

	async fn username(&self) -> String {
		self.username.clone()
	}

	async fn blocks(&self, context: &Context<'_>) -> Result<Vec<Block>, Error> {
		let conn = &context.data::<ContextData>()?.pool.get()?;

		let blocks: Vec<Block> = blocks::dsl::blocks
			.filter(blocks::dsl::owner_id.eq(self.id))
			.load::<BlockD>(conn)?
			.iter()
			.map(Block::from)
			.collect();

		Ok(blocks)
	}
}

impl From<UserD> for User {
	fn from(userd: UserD) -> Self {
		User {
			id: userd.id,
			username: userd.username,
		}
	}
}

pub async fn user_by_id(context: &ContextData, id: i32) -> Result<Option<User>, Error> {
	let conn = &context.pool.get()?;

	let usr: Option<UserD> = users::dsl::users
		.filter(users::id.eq(id))
		.limit(1)
		.get_result(conn)
		.optional()?;

	match usr {
		None => Ok(None),
		Some(usr) => Ok(Some(User::from(usr))),
	}
}

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
	/// How many users there are in the database
	async fn user_count(&self, context: &Context<'_>) -> FieldResult<i32> {
		let conn = &context.data::<ContextData>()?.pool.get()?;

		let num: i64 = users::dsl::users.count().get_result(conn)?;
		Ok(num as i32)
	}

	/// Tries to find a user with a matching ID. Will be null if a user is not found.
	async fn user_by_id(&self, context: &Context<'_>, id: i32) -> Result<Option<User>, Error> {
		let context = &context.data::<ContextData>()?;
		user_by_id(context, id).await
	}

	/// Returns a `User` object corresponding with the authorization token.
	async fn whoami(&self, context: &Context<'_>) -> Result<Option<User>, Error> {
		let context = &context.data::<ContextData>()?;
		let token = require_token(context)?;
		let user_id = validate_token(token)?;

		user_by_id(context, user_id).await
	}
}
