use crate::{block_logic::block::BlockObject, graphql::ContextData};
use async_graphql::*;
use block_tools::{
	auth::{require_token, validate_token},
	dsl::prelude::*,
	models::{Block, User},
	schema::{blocks, users},
};
use strsim::jaro_winkler;

use super::localize_username;

pub struct QLUser {
	/// Auto-incrementing unique ID for a user
	pub id: i32,
	/// Unique alphanumeric username for easy identification
	pub username: String,
	pub display_name: Option<String>,
}

#[Object]
impl QLUser {
	/// How many users there are in the database
	async fn credits(&self, context: &Context<'_>) -> Result<Option<i32>, Error> {
		let context = &context.data::<ContextData>()?;
		let conn = &context.pool.get()?;
		let token = require_token(&context.other())?;

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

	async fn display_name(&self) -> Option<String> {
		self.display_name.clone()
	}

	async fn blocks(&self, context: &Context<'_>) -> Result<Vec<BlockObject>, Error> {
		let conn = &context.data::<ContextData>()?.pool.get()?;

		let blocks: Vec<BlockObject> = blocks::dsl::blocks
			.filter(blocks::dsl::owner_id.eq(self.id))
			.load::<Block>(conn)?
			.iter()
			.map(BlockObject::from)
			.collect();

		Ok(blocks)
	}
}

impl From<User> for QLUser {
	fn from(userd: User) -> Self {
		QLUser {
			id: userd.id,
			username: userd.username,
			display_name: userd.display_name,
		}
	}
}

pub async fn user_by_id(context: &ContextData, id: i32) -> Result<Option<QLUser>, Error> {
	let conn = &context.pool.get()?;

	let usr: Option<User> = users::dsl::users
		.filter(users::id.eq(id))
		.limit(1)
		.get_result(conn)
		.optional()?;

	match usr {
		None => Ok(None),
		Some(usr) => Ok(Some(QLUser::from(usr))),
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
	async fn user_by_id(&self, context: &Context<'_>, id: i32) -> Result<Option<QLUser>, Error> {
		let context = &context.data::<ContextData>()?;
		user_by_id(context, id).await
	}

	/// Tries to find a user with a matching ID. Will be null if a user is not found.
	async fn user_by_name(
		&self,
		context: &Context<'_>,
		username: String,
	) -> Result<Option<QLUser>, Error> {
		let context = &context.data::<ContextData>()?;
		let conn = &context.pool.get()?;

		let localname = localize_username(&username);

		let usr: Option<User> = users::dsl::users
			.filter(users::localuname.eq(localname))
			.limit(1)
			.get_result(conn)
			.optional()?;

		match usr {
			None => Ok(None),
			Some(usr) => Ok(Some(QLUser::from(usr))),
		}
	}

	/// Returns a `User` object corresponding with the authorization token.
	async fn whoami(&self, context: &Context<'_>) -> Result<Option<QLUser>, Error> {
		let context = &context.data::<ContextData>()?;
		let token = require_token(&context.other())?;
		let user_id = validate_token(token)?;

		user_by_id(context, user_id).await
	}

	async fn search_users(
		&self,
		context: &Context<'_>,
		query: String,
	) -> Result<Vec<QLUser>, Error> {
		let context = &context.data::<ContextData>()?;
		let conn = &context.pool.get()?;

		let mut helpers: Vec<UserSortHelper> = users::dsl::users
			.load::<User>(conn)?
			.into_iter()
			.map(|user| {
				let username_sim = jaro_winkler(&user.username, &query);
				let display_name = user.display_name.clone();
				let display_sim = display_name
					.and_then(|name| Some(jaro_winkler(&name, &query)))
					.unwrap_or(0.);
				UserSortHelper {
					user,
					strsim: username_sim.max(display_sim),
				}
			})
			.filter(|helper| helper.strsim != 0.)
			.collect();
		helpers.sort_by(|a, b| b.strsim.partial_cmp(&a.strsim).unwrap());

		let users: Vec<QLUser> = helpers
			.into_iter()
			.map(|helper| helper.user.into())
			.collect();

		Ok(users)
	}
}

struct UserSortHelper {
	user: User,
	strsim: f64,
}
