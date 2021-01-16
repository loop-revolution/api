use crate::{
	graphql::{query::user_by_id, Context},
	user_logic::user::User,
	Error,
};
use block_tools::UserError;
use chrono::{prelude::*, Duration};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use juniper::graphql_object;
use log::error;
use serde::{Deserialize, Serialize};

pub struct AuthPayload {
	pub token: String,
	pub user_id: i32,
}

#[graphql_object(context = Context)]
impl AuthPayload {
	pub async fn user(&self, context: &Context) -> Result<Option<User>, Error> {
		user_by_id(context, self.user_id).await
	}
	fn token(&self) -> String {
		self.token.clone()
	}
}

impl AuthPayload {
	pub fn new(user_id: i32) -> Self {
		AuthPayload {
			user_id,
			token: create_token(user_id),
		}
	}
}

pub fn require_token(context: &Context) -> Result<String, UserError> {
	match &context.auth_token {
		None => Err(UserError::NeedAuth),
		Some(token) => Ok(token.clone()),
	}
}

fn create_token(user_id: i32) -> String {
	let claims = Claims {
		sub: user_id.to_string(),
		iat: Utc::now().timestamp() as usize,
		exp: (Utc::now() + Duration::days(21)).timestamp() as usize,
	};

	encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(get_secret().as_ref()),
	)
	.unwrap()
}

pub fn validate_token(token: String) -> Result<i32, Error> {
	let token = decode::<Claims>(
		&token,
		&DecodingKey::from_secret(get_secret().as_ref()),
		&Validation::default(),
	)?;

	Ok(token.claims.sub.parse()?)
}

fn get_secret() -> String {
	dotenv::dotenv().ok();
	match std::env::var("SESSION_SECRET") {
		Ok(scrt) => scrt,
		Err(_) => {
			error!("A 'SESSION_SECRET' environment variable is required.");
			panic!()
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	sub: String,
	exp: usize,
	iat: usize,
}
