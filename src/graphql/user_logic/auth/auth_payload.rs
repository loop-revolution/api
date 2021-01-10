use crate::graphql::{query::user_by_id, user_logic::user::User, Context, Error};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
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

fn create_token(user_id: i32) -> String {
	let claims = Claims {
		sub: user_id.to_string(),
		iat: Utc::now().timestamp() as usize,
		exp: (Utc::now() + Duration::days(21)).timestamp() as usize,
	};

	encode(&Header::default(), &claims, &get_encoding_key()).unwrap()
}

fn get_encoding_key() -> EncodingKey {
	dotenv::dotenv().ok();
	let session_secret = match std::env::var("SESSION_SECRET") {
		Ok(scrt) => scrt,
		Err(_) => {
			error!("A 'SESSION_SECRET' environment variable is required.");
			panic!()
		}
	};

	EncodingKey::from_secret(session_secret.as_ref())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	sub: String,
	exp: usize,
	iat: usize,
}
