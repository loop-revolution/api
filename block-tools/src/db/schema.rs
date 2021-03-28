table! {
	blocks (id) {
		id -> Int8,
		block_type -> Varchar,
		created_at -> Timestamp,
		updated_at -> Timestamp,
		block_data -> Nullable<Text>,
		owner_id -> Int4,
		public -> Bool,
		perm_full -> Array<Int4>,
		perm_edit -> Array<Int4>,
		perm_view -> Array<Int4>,
		stars -> Array<Int4>,
		notif_enabled -> Array<Int4>,
		color -> Nullable<Varchar>,
	}
}

table! {
	notifications (id) {
		id -> Int8,
		name -> Varchar,
		description -> Varchar,
		block_link -> Nullable<Int8>,
		recipients -> Array<Int4>,
		time -> Nullable<Timestamp>,
	}
}

table! {
	potential_users (id) {
		id -> Int4,
		email -> Varchar,
		session_code -> Varchar,
		verification_code -> Bpchar,
		username -> Varchar,
		password -> Varchar,
		created_at -> Timestamp,
		display_name -> Nullable<Varchar>,
	}
}

table! {
	properties (id) {
		id -> Int8,
		property_name -> Varchar,
		parent_id -> Int8,
		value_id -> Int8,
		annotation -> Nullable<Varchar>,
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
		display_name -> Nullable<Varchar>,
		root_id -> Nullable<Int8>,
		featured_id -> Nullable<Int8>,
		expo_tokens -> Array<Text>,
	}
}

allow_tables_to_appear_in_same_query!(blocks, notifications, potential_users, properties, users,);
