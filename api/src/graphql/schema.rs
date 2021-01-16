use crate::graphql::{mutation::Mutation, query::Query};
use juniper::{EmptySubscription, RootNode};

use super::Context;

/// Typing for the GraphQL Schema
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

/// Assembles the GraphQL Schema
pub fn create_schema() -> Schema {
	Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
