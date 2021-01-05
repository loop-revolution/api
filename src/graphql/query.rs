use super::context::Context;
use juniper::graphql_object;
/// Struct for Juniper to take Query resolvers
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
	async fn get_count(context: &Context) -> i32 {
		let counter = context.counter.lock().await;
		counter.to_owned()
	}
}
