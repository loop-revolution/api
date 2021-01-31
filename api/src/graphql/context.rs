use block_tools::{blocks::Context as ToolsContext, PostgresPool};

/// The context to share among GraphQL requests
pub struct ContextData {
	/// Gives the GraphQL operations access to the DB
	pub pool: PostgresPool,
	pub auth_token: Option<String>,
}

impl ContextData {
	pub fn other(&self) -> ToolsContext {
		ToolsContext {
			pool: self.pool.clone(),
			auth_token: self.auth_token.clone(),
		}
	}
}
