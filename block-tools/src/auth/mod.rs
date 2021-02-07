use crate::{blocks::Context, UserError};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};
pub mod permissions;

pub fn require_token(context: &Context) -> Result<String, UserError> {
	match optional_token(context) {
		None => Err(UserError::NeedAuth),
		Some(token) => Ok(token),
	}
}

pub fn optional_token(context: &Context) -> Option<String> {
	context.auth_token.clone()
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

pub fn optional_validate_token(token: Option<String>) -> Result<Option<i32>, UserError> {
	match token {
		Some(token) => Ok(Some(validate_token(token)?)),
		None => Ok(None),
	}
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
