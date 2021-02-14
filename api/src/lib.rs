pub mod block_logic;
pub mod graphql;
pub mod notifications;
pub mod user_logic;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

/// Generates a random string of length provided
pub fn rand_string(length: usize) -> String {
	let mut rng = thread_rng();
	iter::repeat(())
		.map(|()| rng.sample(Alphanumeric))
		.map(char::from)
		.take(length)
		.collect()
}

#[cfg(test)]
pub mod tests {
	use std::collections::BTreeMap;

	use async_graphql::{Name, Request, Value};
	use block_tools::PostgresPool;

	use crate::graphql::ContextData;
	pub fn value_to_tree(value: &Value) -> &BTreeMap<Name, Value> {
		if let Value::Object(tree) = value {
			return tree;
		} else {
			panic!();
		}
	}

	pub fn panic_val<'a>(tree: &'a BTreeMap<Name, Value>, name: &str) -> &'a Value {
		match tree.get(name) {
			Some(val) => return val,
			None => panic!(),
		}
	}

	pub fn value_of_value<'a>(value: &'a Value, name: &str) -> &'a Value {
		panic_val(value_to_tree(value), name)
	}

	pub fn make_request(query: String, pool: PostgresPool, token: Option<String>) -> Request {
		let mut request = Request::new(query);
		request = request.data(ContextData {
			pool,
			auth_token: token,
		});
		request
	}

	pub fn rem_first_and_last(value: &str) -> &str {
		let mut chars = value.chars();
		chars.next();
		chars.next_back();
		chars.as_str()
	}
}
