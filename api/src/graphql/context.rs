use block_tools::{blocks::Context as ToolsContext, PostgresPool};

/// The context to share among GraphQL requests
pub struct ContextData {
	/// Gives the GraphQL operations access to the DB
	pub pool: PostgresPool,
	pub auth_token: Option<String>,
}

pub fn other_context(context: &ContextData) -> ToolsContext {
	ToolsContext {
		pool: context.pool.clone(),
		auth_token: context.auth_token.clone(),
	}
}
