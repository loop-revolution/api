use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct EmailConfirm {
	pub email: String,
	pub session_code: String,
}
