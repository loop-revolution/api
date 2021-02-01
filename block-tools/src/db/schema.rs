table! {
    blocks (id) {
        id -> Int8,
        block_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        block_data -> Nullable<Text>,
        owner_id -> Int4,
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
    }
}

joinable!(blocks -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    blocks,
    potential_users,
    properties,
    users,
);
