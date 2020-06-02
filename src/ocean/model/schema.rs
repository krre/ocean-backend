table! {
    topics (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
        user_id -> Int4,
        links -> Nullable<Jsonb>,
    }
}

table! {
    user_groups (id) {
        id -> Int4,
        name -> Nullable<Text>,
        code -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Text>,
        token -> Text,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
        group_id -> Int4,
    }
}

joinable!(topics -> users (user_id));
joinable!(users -> user_groups (group_id));

allow_tables_to_appear_in_same_query!(
    topics,
    user_groups,
    users,
);
