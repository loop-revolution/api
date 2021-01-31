use crate::{BlockError, EmailConfirmError, Error, InternalError, UserError};
use std::time::SystemTimeError;

impl From<SystemTimeError> for Error {
	fn from(_: SystemTimeError) -> Self {
		Error::GenericError
	}
}

impl From<InternalError> for Error {
	fn from(e: InternalError) -> Self {
		Error::InternalError(e)
	}
}

impl From<UserError> for Error {
	fn from(e: UserError) -> Self {
		Error::UserError(e)
	}
}

impl From<BlockError> for Error {
	fn from(e: BlockError) -> Self {
		Error::BlockError(e)
	}
}

impl From<EmailConfirmError> for Error {
	fn from(e: EmailConfirmError) -> Self {
		UserError::EmailConfirmError(e).into()
	}
}

impl From<diesel::result::Error> for Error {
	fn from(_: diesel::result::Error) -> Self {
		Error::GenericError
	}
}

impl From<r2d2::Error> for Error {
	fn from(_: r2d2::Error) -> Self {
		InternalError::DatabaseTimeout.into()
	}
}

impl From<std::num::ParseIntError> for UserError {
	fn from(e: std::num::ParseIntError) -> Self {
		match e {
			_ => UserError::JWTGeneric,
		}
	}
}

impl From<serde_json::Error> for BlockError {
	fn from(_: serde_json::Error) -> Self {
		BlockError::InputParse
	}
}

impl From<jsonwebtoken::errors::Error> for UserError {
	fn from(_: jsonwebtoken::errors::Error) -> Self {
		UserError::JWTGeneric
	}
}
