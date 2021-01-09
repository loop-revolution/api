use juniper::{FieldError, IntoFieldError};
use std::{fmt, time::SystemTimeError};

#[derive(Debug, Clone)]
pub enum Error {
	NameConflict(String),
	GenericError,
	PasswordTooShort,
	EmailConfirmError(EmailConfirmError),
	InternalError(InternalError),
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
			Error::PasswordTooShort => write!(f, "[ups] The provided password was too short."),
			Error::GenericError => write!(f, "[g] Something unspecified went wrong."),
			Error::EmailConfirmError(err) => write!(f, "{}", err.to_string()),
			Error::InternalError(err) => write!(f, "{}", err.to_string()),
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
			_ => Error::InternalError(InternalError::DatabaseTimeout),
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

impl From<InternalError> for Error {
	fn from(e: InternalError) -> Self {
		match e {
			_ => Error::InternalError(e),
		}
	}
}

impl From<EmailConfirmError> for Error {
	fn from(e: EmailConfirmError) -> Self {
		match e {
			_ => Error::EmailConfirmError(e),
		}
	}
}

#[derive(Debug, Clone)]
pub enum EmailConfirmError {
	NotFound(String),
	Expired,
	Invalid,
}

impl fmt::Display for EmailConfirmError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			EmailConfirmError::NotFound(name) => write!(
				f,
				"[uecn] An email confirmation with the username \"{}\" was not found.",
				name
			),
			EmailConfirmError::Invalid => write!(
				f,
				"[ueci] The verification code and/or session code were incorrect.",
			),
			EmailConfirmError::Expired => write!(
				f,
				"[uece] The email confirmation has expired because more than 5 minutes has passed since its creation.",
			),
		}
	}
}

#[derive(Debug, Clone)]
pub enum InternalError {
	DatabaseTimeout,
	GenericInternalError,
	EmailError,
}

impl fmt::Display for InternalError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InternalError::DatabaseTimeout => write!(
				f,
				"[idt] There was an issue with connecting to the database."
			),
			InternalError::GenericInternalError => {
				write!(f, "[ig] Something unspecified went wrong internally.",)
			}
			InternalError::EmailError => {
				write!(f, "[img] Something went wrong with Loop's emailing system.")
			}
		}
	}
}
