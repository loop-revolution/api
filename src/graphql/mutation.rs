use super::{
	context::Context,
	models::{NewUser, User},
	user_logic::{hash_pwd, localize_username, verify_pwd},
	Error,
};
use crate::db::schema::users;
use diesel::prelude::*;
use juniper::graphql_object;

/// Struct for GraphQL Mutations
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
	/// Attempts to create an account for the username provided
	async fn signup(context: &Context, username: String, password: String) -> Result<User, Error> {
		verify_pwd(&password)?;
		// Hash the password
		let hash = hash_pwd(password)?;

		// Model the user to insert
		let new_user = NewUser {
			username: &username,
			password: &hash,
			localuname: &*localize_username(&username),
		};

		let conn = &context.pool.get()?;

		// Preform the insertion to the DB
		match diesel::insert_into(users::table)
			.values(&new_user)
			.get_result(conn)
		{
			Ok(usr) => Ok(usr),
			_ => Err(Error::NameConflict(username)),
		}
	}
}
