table! {
    topics (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    topics,
    users,
);
