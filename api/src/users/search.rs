use crate::graphql::ContextData;
use async_graphql::*;
use block_tools::{dsl::prelude::*, models::User, schema::users};
use strsim::jaro_winkler;

use super::user::UserObject;

#[derive(Default)]
pub struct UserSearchQueries;

#[Object]
impl UserSearchQueries {
	/// Finds users that are similar to the query provided. Matches against
	/// both username and display name and sorts them by similarity.
	async fn search_users(
		&self,
		context: &Context<'_>,
		query: String,
	) -> Result<Vec<UserObject>, Error> {
		let (_, conn) = &ContextData::parse(context)?;

		// Vector of user & their score pairs
		let mut helpers: Vec<UserSortHelper> = users::dsl::users
			.load::<User>(conn)?
			.into_iter()
			// Generate the similarity scores for each user
			.map(|user| {
				// Similarity scores for both username and display name
				let username_sim = jaro_winkler(&user.username, &query);
				let display_sim = user
					.display_name
					.clone()
					.map(|name| jaro_winkler(&name, &query))
					.unwrap_or_default();
				// Helper pair
				(user, username_sim.max(display_sim))
			})
			// Remove all the names with a similarity of 0
			.filter(|helper| helper.1 != 0.)
			.collect();
		// Sort the user results
		helpers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

		// Turn the helpers back into the single users
		Ok(helpers.into_iter().map(|helper| helper.0.into()).collect())
	}
}

type UserSortHelper = (User, f64);
