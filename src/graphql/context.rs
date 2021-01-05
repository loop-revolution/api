use super::super::db::PostgresPool;
use std::sync::Arc;
use tokio::sync::Mutex;

/// The context to share among GraphQL requests
pub struct Context {
	/// Gives the GraphQL operations access to the DB
	pub pool: PostgresPool,
	// Count
	pub counter: Arc<Mutex<i32>>,
}

// Passes the context to GraphQL Objects in Juniper
impl juniper::Context for Context {}
