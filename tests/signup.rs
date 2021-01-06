use juniper::{graphql_value, Variables};
use loop_api::{
	db::{env_db, get_pool},
	graphql::{create_schema, Context},
};

#[tokio::test]
async fn counting() {
	// Setups
	let schema = create_schema();
	let pool = get_pool(&env_db());
	let context = Context { pool: pool.clone() };

	// Sign up
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
}
