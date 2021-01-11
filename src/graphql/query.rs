use super::{context::Context, models::UserD, user_logic::user::User};
use crate::{db::schema::users, Error};
use diesel::prelude::*;
use juniper::{graphql_object, FieldResult};
/// Struct for Juniper to take Query resolvers
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
	/// How many users there are in the database
	async fn user_count(context: &Context) -> FieldResult<i32> {
		let conn = &context.pool.get()?;

		let num: i64 = users::dsl::users.count().get_result(conn)?;
		Ok(num as i32)
	}

	async fn user_by_id(context: &Context, id: i32) -> Result<Option<User>, Error> {
		user_by_id(context, id).await
	}
}

pub async fn user_by_id(context: &Context, id: i32) -> Result<Option<User>, Error> {
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
