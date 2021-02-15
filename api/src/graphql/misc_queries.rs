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
	use crate::{
		graphql::build_schema,
		tests::{expect_tree_val, rem_first_and_last},
	};

	#[tokio::test]
	async fn api_version() {
		let expecter_ver = env!("CARGO_PKG_VERSION").to_string();
		let res = build_schema().execute("{ apiVersion }").await.data;
		let version = expect_tree_val(&res, "apiVersion").to_string();
		assert_eq!(rem_first_and_last(&version), expecter_ver);
	}
}
