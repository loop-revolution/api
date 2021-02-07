use crate::{
	block_logic::block_queries::{BlockMutations, BlockQueries},
	notifications::{NotificationMutations, Notifications},
	user_logic::{
		auth::{login::LoginMutations, signup::SignupMutations},
		user::UserQueries,
	},
};
use async_graphql::{MergedObject, Schema as GraphQLSchema};

use super::misc_queries::MiscQueries;

#[derive(MergedObject, Default)]
pub struct Query(UserQueries, BlockQueries, MiscQueries);

#[derive(MergedObject, Default)]
pub struct Mutation(
	SignupMutations,
	LoginMutations,
	BlockMutations,
	NotificationMutations,
);

pub type Schema = GraphQLSchema<Query, Mutation, Notifications>;

pub fn build_schema() -> Schema {
	Schema::build(Query::default(), Mutation::default(), Notifications).finish()
}
