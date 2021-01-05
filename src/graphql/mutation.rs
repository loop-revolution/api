use super::context::Context;
use juniper::graphql_object;

/// Struct for GraphQL Mutations
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
	/// Modifies the internal counter
	async fn change_count(context: &Context, by: i32) -> i32 {
		let mut counter = context.counter.lock().await;
		*counter += by;
		counter.to_owned()
	}
}
