use async_graphql::*;

#[derive(SimpleObject)]
pub struct EmailConfirm {
	pub email: String,
	pub session_code: String,
}
