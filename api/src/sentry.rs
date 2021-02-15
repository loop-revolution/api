use block_tools::sentry;

pub fn sentry() -> Option<sentry::ClientInitGuard> {
	let dsn = match std::env::var("SENTRY_DSN") {
		Ok(dsn) => dsn,
		Err(_) => return None,
	};
	Some(sentry::init((
		dsn.as_str(),
		sentry::ClientOptions {
			release: sentry::release_name!(),
			..Default::default()
		},
	)))
}
