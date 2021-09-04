use super::misc_queries::MiscQueries;
use crate::{
	blocks::{
		basic::{BasicBlockMutations, BasicBlockQueries},
		colors::BlockColorMutations,
		comments::CommentMutations,
		create::{BlockCreationMutation, BlockCreationQuery},
		perms::BlockPermMutations,
		search::BlockSearchQueries,
	},
	notifications::{
		queries::NotificationQueries,
		sub::Notifications,
		updates::{UpdateMutations, UpdateQueries},
		NotificationMutations,
	},
	users::{
		auth::{
			confirm_email::ConfirmEmailMutation, forgot_password::ForgotPasswordMutations,
			login::LoginMutations, signup::SignupMutations, update_email::UpdateEmailMutation,
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
	UpdateQueries,
	UserQueries,
	UserSearchQueries,
	UserSelectingQueries,
);

#[derive(MergedObject, Default)]
pub struct Mutation(
	BasicBlockMutations,
	BlockCreationMutation,
	BlockPermMutations,
	CommentMutations,
	ConfirmEmailMutation,
	ForgotPasswordMutations,
	LoginMutations,
	NotificationMutations,
	SignupMutations,
	SpecialBlockMutations,
	UpdateEmailMutation,
	UpdateMutations,
	UserInfoMutations,
	BlockColorMutations,
);

pub type Schema = GraphQLSchema<Query, Mutation, Notifications>;

/// Combines all the GraphQL resolvers into a schema
pub fn build_schema() -> Schema {
	Schema::build(Query::default(), Mutation::default(), Notifications).finish()
}
