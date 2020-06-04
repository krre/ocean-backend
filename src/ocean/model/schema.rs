table! {
    mandels (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        user_id -> Int4,
        images -> Jsonb,
        videos -> Jsonb,
        links -> Jsonb,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
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
        group_id -> Int4,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
    }
}

joinable!(mandels -> users (user_id));
joinable!(users -> user_groups (group_id));

allow_tables_to_appear_in_same_query!(
    mandels,
    user_groups,
    users,
);
