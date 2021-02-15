use crate::{
	graphql::build_schema,
	rand_string,
	tests::{build_request, expect_tree_val, rem_first_and_last},
};
use block_tools::{
	auth::validate_token, dsl::prelude::*, env_db, get_pool, schema::potential_users,
};

#[tokio::test]
async fn password_too_short() {
	let schema = build_schema();

	// Try to signup with a short password
	let query = "mutation { signup (
					username: \"name\",
					password: \"pwd\",
					email: \"fake@e.mail\",
				) { sessionCode } }";
	let res = schema.execute(query).await;

	assert!(res.errors[0].message.contains("[ups]"));
}

#[tokio::test]
async fn successful_signup() {
	let pool = get_pool(&env_db());
	let schema = build_schema();
	let username = rand_string(10);
	let password = rand_string(10);
	let request = build_request(
		format!(
			r#"mutation {{signup (username: "{}", password: "{}", email: "fake@e.mail") {{ sessionCode }} }}"#,
			username, password
		),
		pool.clone(),
		None,
	);
	let data = schema.execute(request).await.data;
	let data = expect_tree_val(&data, "signup");
	let session_code = expect_tree_val(data, "sessionCode").to_string();
	let session_code = rem_first_and_last(&session_code);
	let conn = pool.get().unwrap();
	let verification_code: String = potential_users::dsl::potential_users
		.filter(potential_users::username.eq(&username))
		.select(potential_users::verification_code)
		.first(&conn)
		.unwrap();
	let request = build_request(
		format!(
			r#"mutation {{ confirmEmail (username: "{}", sessionCode: "{}", verificationCode: "{}") {{ token, user {{ username }} }} }}"#,
			username, session_code, verification_code,
		),
		pool.clone(),
		None,
	);
	let data = schema.execute(request).await;
	let data = data.data;
	let data = expect_tree_val(&data, "confirmEmail");
	let token = expect_tree_val(data, "token").to_string();
	let token = rem_first_and_last(&token);
	validate_token(&token).unwrap();
	let user = expect_tree_val(data, "user");
	let resulting_username = expect_tree_val(user, "username").to_string();
	let resulting_username = rem_first_and_last(&resulting_username);
	assert_eq!(resulting_username, username);
}
