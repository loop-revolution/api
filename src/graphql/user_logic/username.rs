use regex::Regex;

pub fn localize_username<'a>(username: &'a str) -> String {
	let re = Regex::new(r"[^a-zA-Z\d]").unwrap();
	re.replace_all(username, "").to_string().to_lowercase()
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
