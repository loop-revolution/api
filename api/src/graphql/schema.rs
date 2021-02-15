use super::misc_queries::MiscQueries;
use crate::{
	blocks::{
		basic::{BasicBlockMutations, BasicBlockQueries},
		create::{BlockCreationMutation, BlockCreationQuery},
		perms::BlockPermMutations,
		search::BlockSearchQueries,
	},
	notifications::{queries::NotificationQueries, sub::Notifications, NotificationMutations},
	users::{
		auth::{
			confirm_email::ConfirmEmailMutation, login::LoginMutations, signup::SignupMutations,
		},
		info::user_info::UserInfoMutations,
		search::UserSearchQueries,
		selecting::UserSelectingQueries,
		special::SpecialBlockMutations,
		user::UserQueries,
	},
};
use async_graphql::{MergedObject, Schema as GraphQLSchema};

#[derive(MergedObject, Default)]
pub struct Query(
	BasicBlockQueries,
	BlockCreationQuery,
	BlockSearchQueries,
	MiscQueries,
	NotificationQueries,
	UserQueries,
	UserSearchQueries,
	UserSelectingQueries,
);

#[derive(MergedObject, Default)]
pub struct Mutation(
	BasicBlockMutations,
	BlockCreationMutation,
	BlockPermMutations,
	ConfirmEmailMutation,
	LoginMutations,
	NotificationMutations,
	SignupMutations,
	SpecialBlockMutations,
	UserInfoMutations,
);

pub type Schema = GraphQLSchema<Query, Mutation, Notifications>;

/// Combines all the GraphQL resolvers into a schema
pub fn build_schema() -> Schema {
	Schema::build(Query::default(), Mutation::default(), Notifications).finish()
}
