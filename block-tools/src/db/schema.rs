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
	comments (id) {
		id -> Int8,
		author_id -> Int4,
		content_id -> Int8,
		block_id -> Int8,
		stars -> Array<Int4>,
		created_at -> Timestamp,
	}
}

table! {
	email_confirm (id) {
		id -> Int4,
		new_email -> Varchar,
		session_code -> Varchar,
		verification_code -> Bpchar,
		user_id -> Int4,
		created_at -> Timestamp,
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
	updates (id) {
		id -> Int4,
		created_at -> Timestamp,
		display -> Text,
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
		latest_update_seen_id -> Nullable<Int4>,
	}
}

joinable!(comments -> blocks (content_id));
joinable!(comments -> users (author_id));
joinable!(email_confirm -> users (user_id));
joinable!(users -> updates (latest_update_seen_id));

allow_tables_to_appear_in_same_query!(
	blocks,
	comments,
	email_confirm,
	notifications,
	potential_users,
	properties,
	updates,
	users,
);
