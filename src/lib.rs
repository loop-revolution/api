#[macro_use]
extern crate diesel;

pub mod db;
pub mod graphql;
pub use graphql::Error;

#[cfg(test)]
mod test {
	use crate::{
		db::{env_db, get_pool},
		graphql::{create_schema, Context, Schema},
		Error,
	};
	use juniper::{DefaultScalarValue, ExecutionError, Value, Variables};
	use std::time::SystemTime;

	/// Generates a "random" username based on time
	pub fn rand_username() -> Result<String, Error> {
		Ok(SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)?
			.as_millis()
			.to_string())
	}

	/// Makes a Schema & Context instance for integration tests
	pub fn gen_exec() -> (Context, Schema) {
		let schema = create_schema();
		let pool = get_pool(&env_db());
		let context = Context { pool: pool.clone() };
		(context, schema)
	}

	pub async fn easy_exec<'a>(
		query: &'a str,
		pair: (&Schema, &Context),
	) -> (
		Value<DefaultScalarValue>,
		Vec<ExecutionError<DefaultScalarValue>>,
	) {
		juniper::execute(query, None, pair.0, &Variables::new(), pair.1)
			.await
			.unwrap()
	}
}
