use db::{DieselError, R2D2Error};
use errors::{EmailConfirmError, Error as RootError, InternalError, UserError};
use juniper::{FieldError, IntoFieldError};
use std::{fmt, time::SystemTimeError};

#[derive(Debug, Clone)]
pub enum Error {
	RootError(RootError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::RootError(err) => write!(f, "{}", err.to_string()),
		}
	}
}

impl<S> IntoFieldError<S> for Error {
	fn into_field_error(self) -> FieldError<S> {
		FieldError::new(self.to_string(), juniper::Value::Null)
	}
}

impl From<argon2::Error> for Error {
	fn from(e: argon2::Error) -> Self {
		match e {
			_ => Error::RootError(RootError::GenericError),
		}
	}
}

impl From<jsonwebtoken::errors::Error> for Error {
	fn from(e: jsonwebtoken::errors::Error) -> Self {
		match e {
			_ => Error::RootError(UserError::JWTGeneric.into()),
		}
	}
}

impl From<std::num::ParseIntError> for Error {
	fn from(e: std::num::ParseIntError) -> Self {
		match e {
			_ => Error::RootError(UserError::JWTGeneric.into()),
		}
	}
}

impl From<DieselError> for Error {
	fn from(e: DieselError) -> Self {
		match e {
			_ => Error::RootError(RootError::GenericError),
		}
	}
}

impl From<R2D2Error> for Error {
	fn from(e: R2D2Error) -> Self {
		match e {
			_ => Error::RootError(RootError::InternalError(InternalError::DatabaseTimeout)),
		}
	}
}

impl From<RootError> for Error {
	fn from(e: RootError) -> Self {
		match e {
			_ => Error::RootError(e),
		}
	}
}

impl From<UserError> for Error {
	fn from(e: UserError) -> Self {
		match e {
			_ => Error::RootError(e.into()),
		}
	}
}

impl From<InternalError> for Error {
	fn from(e: InternalError) -> Self {
		match e {
			_ => Error::RootError(e.into()),
		}
	}
}

impl From<EmailConfirmError> for Error {
	fn from(e: EmailConfirmError) -> Self {
		match e {
			_ => Error::RootError(e.into()),
		}
	}
}

impl From<SystemTimeError> for Error {
	fn from(e: SystemTimeError) -> Self {
		match e {
			_ => Error::RootError(e.into()),
		}
	}
}
