use super::context::Context;
use crate::db::schema::users;
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
}
