use db::{env_db, get_pool};
use loop_api::graphql::{create_schema, Context};
use std::env;
use warp::{
	http::{header, Method},
	Filter,
};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
	// Logging
	env::set_var("RUST_LOG", "loop_api=info,loop_api=debug");
	pretty_env_logger::init();
	let log = warp::log::custom(|info| {
		info!("{} in {:?}", info.status(), info.elapsed());
	});

	// Create a database pool to Postgres
	let private_pool = get_pool(&env_db());
	let public_pool = get_pool(&env_db());
	// This creates the graphql context for each request
	let state = warp::any()
		.and(
			warp::header::<String>("authorization").map(move |mut bearer_token: String| Context {
				pool: private_pool.clone(),
				auth_token: Some(bearer_token.split_off(7)),
			}),
		)
		.or(warp::any().map(move || Context {
			pool: public_pool.clone(),
			auth_token: None,
		}))
		.unify();
	// This is the connection to Juniper, using the schema & context
	let graphql_filter = juniper_warp::make_graphql_filter(create_schema(), state.boxed());
	let cors = warp::cors()
		.allow_methods(&[Method::GET, Method::POST, Method::OPTIONS])
		.allow_any_origin()
		.allow_headers(&[header::CONTENT_TYPE, header::AUTHORIZATION]);
	// Filter `/graphql` to the GraphQL API
	let api = warp::path("graphql").and(graphql_filter);
	// The interactive GraphQL playground present at `/play`
	let playground = warp::get()
		.and(warp::path("play"))
		.and(juniper_warp::playground_filter("/graphql", None));

	// Use all the routes, connect logging
	let routes = playground.or(api).with(log);

	// Use PORT variable default to 4000
	let port = match env::var("PORT") {
		Ok(str) => str.parse().unwrap(),
		_ => 4000,
	};

	// Announce the server is online
	log::info!("Listening on 0.0.0.0:{}", port);

	warp::serve(routes.with(cors))
		.run(([0, 0, 0, 0], port))
		.await;
}
