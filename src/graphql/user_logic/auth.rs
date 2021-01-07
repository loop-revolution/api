use super::{hash_pwd, localize_username, verify_pwd};
use crate::{
	db::schema::users,
	graphql::{
		models::{NewUser, User},
		Context,
	},
	Error,
};
use diesel::prelude::*;

pub async fn signup(context: &Context, username: String, password: String) -> Result<User, Error> {
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

#[cfg(test)]
mod test {
	use crate::test::{easy_exec, gen_exec, rand_username};
	use juniper::graphql_value;

	#[tokio::test]
	async fn sign_up_successful() {
		let (context, schema) = gen_exec();
		let name = rand_username().unwrap();

		let query = format!(
			"mutation {{ signup (
					username: \"{}\",
					password: \"badpassword\",
				) {{ username }} }}",
			name
		);

		let (res, _) = easy_exec(&query, (&schema, &context)).await;

		assert_eq!(
			res,
			graphql_value!({
				"signup": {
					"username": name,
				},
			})
		);
	}

	#[tokio::test]
	async fn sign_up_name_conflict() {
		let (context, schema) = gen_exec();
		let name = rand_username().unwrap();

		// Step 1: Make the account with the name
		let query = format!(
			"mutation {{ signup (
					username: \"{}\",
					password: \"badpassword\",
				) {{ username }} }}",
			name
		);

		let (res, _) = easy_exec(&query, (&schema, &context)).await;

		assert_eq!(
			res,
			graphql_value!({
				"signup": {
					"username": name,
				},
			})
		);

		// Step 2: Try to make an account with the same name
		let (_, errors) = easy_exec(&query, (&schema, &context)).await;

		assert!(&errors[0].error().message().contains("[unc]"));
	}

	#[tokio::test]
	async fn password_too_short() {
		let (context, schema) = gen_exec();

		// Try to signup with a short password
		let query = "mutation { signup (
					username: \"name\",
					password: \"pwd\",
				) { username } }";
		let (_, errors) = easy_exec(&query, (&schema, &context)).await;

		assert!(&errors[0].error().message().contains("[ups]"));
	}
}
