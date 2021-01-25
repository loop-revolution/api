use block_tools::{blocks::Context as ToolsContext, PostgresPool};

/// The context to share among GraphQL requests
pub struct Context {
	/// Gives the GraphQL operations access to the DB
	pub pool: PostgresPool,
	pub auth_token: Option<String>,
}

// Passes the context to GraphQL Objects in Juniper
impl juniper::Context for Context {}

pub fn other_context(context: &Context) -> ToolsContext {
	ToolsContext {
		pool: context.pool.clone(),
		auth_token: context.auth_token.clone(),
	}
}
