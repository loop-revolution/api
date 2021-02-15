use block_tools::{blocks::Context as ToolsContext, PgConnect, PostgresPool};

/// The context to share among GraphQL requests
pub struct ContextData {
	/// Gives the GraphQL operations access to the DB
	pub pool: PostgresPool,
	/// A JWT for authenticating a user with a request
	pub auth_token: Option<String>,
}

impl ContextData {
	/// Converts this context into the one that block_tools uses
	pub fn other(&self) -> ToolsContext {
		ToolsContext {
			pool: self.pool.clone(),
			auth_token: self.auth_token.clone(),
		}
	}

	/// Shorthand for getting context and connection from the GraphQL context
	pub fn parse(
		context: &async_graphql::Context<'_>,
	) -> async_graphql::Result<(ToolsContext, PgConnect)> {
		let context = context.data::<ContextData>()?.other();
		let conn = context.conn()?;
		Ok((context, conn))
	}
}
