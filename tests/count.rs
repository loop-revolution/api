use juniper::{graphql_value, Variables};
use loop_api::{
	db::{env_db, get_pool},
	graphql::{create_schema, Context},
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn counting() {
	// Setups
	let counter = Arc::new(Mutex::<i32>::new(0));
	let schema = create_schema();
	let pool = get_pool(&env_db());
	let context = Context {
		pool: pool.clone(),
		counter: Arc::clone(&counter),
	};

	// Query the count (initially)
	let (res, _) = juniper::execute(
		"query { getCount }",
		None,
		&schema,
		&Variables::new(),
		&context,
	)
	.await
	.unwrap();

	// The initial count should be 0
	assert_eq!(
		res,
		graphql_value!({
			"getCount": 0,
		})
	);

	// Add 5 to the count
	let (res, _) = juniper::execute(
		"mutation { changeCount(by: 5) }",
		None,
		&schema,
		&Variables::new(),
		&context,
	)
	.await
	.unwrap();

	// After adding 5, the count should be 5.
	assert_eq!(
		res,
		graphql_value!({
			"changeCount": 5,
		})
	);

	// Add another 3...
	let (res, _) = juniper::execute(
		"mutation { changeCount(by: 3) }",
		None,
		&schema,
		&Variables::new(),
		&context,
	)
	.await
	.unwrap();

	// ...to get 8
	assert_eq!(
		res,
		graphql_value!({
			"changeCount": 8,
		})
	);
}
