use crate::{blocks::Context, UserError};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};

pub fn require_token(context: &Context) -> Result<String, UserError> {
	match &context.auth_token {
		None => Err(UserError::NeedAuth),
		Some(token) => Ok(token.clone()),
	}
}

pub fn create_token(user_id: i32) -> String {
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

pub fn validate_token(token: String) -> Result<i32, UserError> {
	let token = decode::<Claims>(
		&token,
		&DecodingKey::from_secret(get_secret().as_ref()),
		&Validation::default(),
	)?;

	Ok(token.claims.sub.parse()?)
}

pub fn get_secret() -> String {
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
pub struct Claims {
	sub: String,
	exp: usize,
	iat: usize,
}
