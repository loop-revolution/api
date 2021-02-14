use async_graphql::*;

#[derive(Default)]
pub struct MiscQueries {}

#[Object]
impl MiscQueries {
	/// Returns the api version as a string.
	/// The version string returned is exactly the Rust
	/// crate version for `loop-api`.
	async fn api_version(&self) -> String {
		env!("CARGO_PKG_VERSION").to_string()
	}
}

#[cfg(test)]
mod tests {
	use async_graphql::Value;

	use crate::{graphql::build_schema, tests::value_of_value};

	#[tokio::test]
	async fn api_version() {
		let expecter_ver = env!("CARGO_PKG_VERSION").to_string();
		let schema = build_schema();
		let res = schema.execute("{ apiVersion }").await.data;
		let version = value_of_value(&res, "apiVersion");
		assert_eq!(version, &Value::from(expecter_ver));
	}
}
