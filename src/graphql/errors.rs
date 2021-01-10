use juniper::{FieldError, IntoFieldError};
use std::{fmt, time::SystemTimeError};

#[derive(Debug, Clone)]
pub enum Error {
	GenericError,
	InternalError(InternalError),
	UserError(UserError)
}

impl<S> IntoFieldError<S> for Error {
	fn into_field_error(self) -> FieldError<S> {
		FieldError::new(self.to_string(), juniper::Value::Null)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::GenericError => write!(f, "[g] Something unspecified went wrong."),
			Error::InternalError(err) => write!(f, "{}", err.to_string()),
			Error::UserError(err) => write!(f, "{}", err.to_string()),
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

impl From<UserError> for Error {
	fn from(e: UserError) -> Self {
		match e {
			_ => Error::UserError(e),
		}
	}
}

impl From<EmailConfirmError> for Error {
	fn from(e: EmailConfirmError) -> Self {
		match e {
			_ => UserError::EmailConfirmError(e).into(),
		}
	}
}

impl From<jsonwebtoken::errors::Error> for Error {
	fn from(e: jsonwebtoken::errors::Error) -> Self {
		match e {
			_ => UserError::JWTGeneric.into(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum UserError {
	PasswordTooShort,
	PasswordMatch,
	EmailConfirmError(EmailConfirmError),
	NameNonexist(String),
	NameConflict(String),
	JWTGeneric,
}

impl fmt::Display for UserError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			UserError::NameConflict(name) => {
				write!(f, "[unc] Username '{}' is already in use.", name)
			},
			UserError::NameNonexist(name) => {
				write!(f, "[une] A user with the username '{}' was not found.", name)
			},
			UserError::PasswordTooShort => write!(f, "[ups] The password provided was too short."),
			UserError::PasswordMatch => write!(f, "[upm] The password provided is not correct."),
			UserError::EmailConfirmError(err) => write!(f, "{}", err.to_string()),
			UserError::JWTGeneric => write!(f, "[ujg] Something unspecified went wrong with user sessions.")
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
