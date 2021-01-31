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
