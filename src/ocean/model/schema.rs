table! {
    comments (id) {
        id -> Int4,
        mandela_id -> Int4,
        user_id -> Int4,
        message -> Text,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
    }
}

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
        title_mode -> Int4,
        what -> Text,
        before -> Text,
        after -> Text,
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

joinable!(comments -> mandels (mandela_id));
joinable!(comments -> users (user_id));
joinable!(mandels -> users (user_id));
joinable!(users -> user_groups (group_id));

allow_tables_to_appear_in_same_query!(
    comments,
    mandels,
    user_groups,
    users,
);
