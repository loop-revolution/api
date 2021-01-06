use super::super::db::PostgresPool;

/// The context to share among GraphQL requests
pub struct Context {
	/// Gives the GraphQL operations access to the DB
	pub pool: PostgresPool,
}

// Passes the context to GraphQL Objects in Juniper
impl juniper::Context for Context {}
