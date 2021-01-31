pub mod block_logic;
pub mod graphql;
pub mod user_logic;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

/// Generates a random string of length provided
pub fn rand_string(length: usize) -> String {
	let mut rng = thread_rng();
	iter::repeat(())
		.map(|()| rng.sample(Alphanumeric))
		.map(char::from)
		.take(length)
		.collect()
}

#[cfg(test)]
mod test {
	use async_graphql::Error;
	use std::time::SystemTime;

	/// Generates a "random" username based on time
	pub fn _rand_username() -> Result<String, Error> {
		Ok(SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)?
			.as_millis()
			.to_string())
	}
}
