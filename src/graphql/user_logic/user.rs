use crate::{
	db::schema::users,
	graphql::{models::UserD, Context},
	Error,
};
use diesel::prelude::*;
use juniper::graphql_object;

pub struct User {
	/// Auto-incrementing unique ID for a user
	pub id: i32,
	/// Unique alphanumeric username for easy identification
	pub username: String,
}

#[graphql_object(context = Context)]
impl User {
	/// How many users there are in the database
	async fn credits(&self, context: &Context) -> Result<i32, Error> {
		let conn = &context.pool.get()?;

		Ok(users::dsl::users
			.filter(users::id.eq(self.id))
			.select(users::credits)
			.first(conn)?)
	}

	fn id(&self) -> i32 {
		self.id
	}

	fn username(&self) -> String {
		self.username.clone()
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
