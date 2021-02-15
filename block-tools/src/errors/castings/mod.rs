use crate::{BlockError, EmailConfirmError, Error, InternalError, UserError};
use std::time::SystemTimeError;

impl From<SystemTimeError> for Error {
	fn from(e: SystemTimeError) -> Self {
		sentry::capture_error(&e);
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
	fn from(e: diesel::result::Error) -> Self {
		sentry::capture_error(&e);
		Error::GenericError
	}
}

impl From<r2d2::Error> for Error {
	fn from(e: r2d2::Error) -> Self {
		sentry::capture_error(&e);
		InternalError::DatabaseTimeout.into()
	}
}

impl From<std::num::ParseIntError> for UserError {
	fn from(_: std::num::ParseIntError) -> Self {
		UserError::JwtGeneric
	}
}

impl From<serde_json::Error> for BlockError {
	fn from(_: serde_json::Error) -> Self {
		BlockError::InputParse
	}
}

impl From<jsonwebtoken::errors::Error> for UserError {
	fn from(_: jsonwebtoken::errors::Error) -> Self {
		UserError::JwtGeneric
	}
}
