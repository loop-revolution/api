use async_graphql::Error;
use block_tools::{
	dsl::{prelude::*, select},
	schema::users::dsl,
	PgConnect, UserError,
};
use regex::Regex;

/// Turns a username into something more standard. Returns non-alphanumeric chars and
/// makes it lowercase
pub fn localize_username(username: &'_ str) -> String {
	let re = Regex::new(r"[^a-zA-Z\d]").unwrap();
	re.replace_all(username, "").to_string().to_lowercase()
}

/// Makes sure that a username is valid and not already used
pub fn validate_username(localuname: &'_ str, conn: &PgConnect) -> Result<(), Error> {
	if localuname.len() < 3 {
		return Err(UserError::NameTooShort(localuname.to_string()).into());
	}
	// A user with that name should not exist!
	let name_exists: bool = select(block_tools::dsl::dsl::exists(
		dsl::users.filter(dsl::localuname.eq(localuname)),
	))
	.get_result(conn)?;
	if name_exists {
		return Err(UserError::NameConflict(localuname.to_string()).into());
	};
	Ok(())
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn localize_redundant() {
		assert_eq!("louis", localize_username("louis"));
	}

	#[test]
	fn localize_case() {
		assert_eq!("loop", localize_username("Loop"));
	}

	#[test]
	fn localize_numbers() {
		assert_eq!("number1", localize_username("Number1"));
	}

	#[test]
	fn localize_special() {
		assert_eq!("extracool", localize_username("EXTRA-COOL"));
	}
}
