use super::{context::Context, models::User, user_logic::auth::signup, Error};
use juniper::graphql_object;

/// Struct for GraphQL Mutations
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
	/// Attempts to create an account for the username provided
	async fn signup(context: &Context, username: String, password: String) -> Result<User, Error> {
		signup(context, username, password).await
	}
}
