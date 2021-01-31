use crate::{
	block_logic::block_queries::{BlockMutations, BlockQueries},
	user_logic::{
		auth::{login::LoginMutations, signup::SignupMutations},
		user::UserQueries,
	},
};
use async_graphql::{EmptySubscription, MergedObject, Schema as GraphQLSchema};

use super::misc_queries::MiscQueries;

#[derive(MergedObject, Default)]
pub struct Query(UserQueries, BlockQueries, MiscQueries);

#[derive(MergedObject, Default)]
pub struct Mutation(SignupMutations, LoginMutations, BlockMutations);

pub type Schema = GraphQLSchema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> Schema {
	Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish()
}
