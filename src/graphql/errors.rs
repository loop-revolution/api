use std::{fmt, time::SystemTimeError};

use juniper::{FieldError, IntoFieldError};

#[derive(Debug, Clone)]
pub enum Error {
	NameConflict(String),
	DatabaseTimeout,
	GenericError,
	PasswordTooShort,
}

impl<S> IntoFieldError<S> for Error {
	fn into_field_error(self) -> FieldError<S> {
		FieldError::new(self.to_string(), juniper::Value::Null)
	}
}

pub fn jfe(error: Error) -> FieldError {
	FieldError::from(error)
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::NameConflict(name) => {
				write!(f, "[unc] Username '{}' is already in use.", name)
			}
			Error::DatabaseTimeout => write!(
				f,
				"[sd] There was an issue with connecting to the database."
			),
			Error::PasswordTooShort => write!(f, "[ups] The provided password was too short."),
			Error::GenericError => write!(f, "[g] Something unspecified went wrong."),
		}
	}
}

impl From<diesel::result::Error> for Error {
	fn from(e: diesel::result::Error) -> Self {
		match e {
			_ => Error::GenericError,
		}
	}
}

impl From<r2d2::Error> for Error {
	fn from(e: r2d2::Error) -> Self {
		match e {
			_ => Error::DatabaseTimeout,
		}
	}
}

impl From<SystemTimeError> for Error {
	fn from(e: SystemTimeError) -> Self {
		match e {
			_ => Error::GenericError,
		}
	}
}

impl From<argon2::Error> for Error {
	fn from(e: argon2::Error) -> Self {
		match e {
			_ => Error::GenericError,
		}
	}
}
