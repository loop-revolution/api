table! {
	potential_users (id) {
		id -> Int4,
		email -> Varchar,
		session_code -> Varchar,
		verification_code -> Bpchar,
		username -> Varchar,
		password -> Varchar,
		created_at -> Timestamp,
	}
}

table! {
	users (id) {
		id -> Int4,
		username -> Varchar,
		localuname -> Varchar,
		password -> Varchar,
		email -> Varchar,
		credits -> Int4,
	}
}

allow_tables_to_appear_in_same_query!(potential_users, users,);
