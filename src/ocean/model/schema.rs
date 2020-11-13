table! {
    categories (id) {
        id -> Int4,
        mandela_id -> Int4,
        number -> Int2,
    }
}

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
    forum_categories (id) {
        id -> Int4,
        name -> Text,
        order_index -> Int2,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
    }
}

table! {
    forum_sections (id) {
        id -> Int4,
        category_id -> Int4,
        name -> Text,
        order_index -> Int2,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
    }
}

table! {
    forum_topics (id) {
        id -> Int4,
        section_id -> Int4,
        user_id -> Int4,
        name -> Text,
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
    marks (id) {
        id -> Int4,
        mandela_id -> Int4,
        user_id -> Int4,
        create_ts -> Timestamptz,
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

table! {
    values (id) {
        id -> Int4,
        name -> Text,
        value -> Nullable<Jsonb>,
    }
}

table! {
    votes (id) {
        id -> Int4,
        mandela_id -> Int4,
        user_id -> Int4,
        vote -> Int2,
        create_ts -> Timestamptz,
    }
}

joinable!(categories -> mandels (mandela_id));
joinable!(comments -> mandels (mandela_id));
joinable!(comments -> users (user_id));
joinable!(forum_sections -> forum_categories (category_id));
joinable!(forum_topics -> forum_sections (section_id));
joinable!(forum_topics -> users (user_id));
joinable!(mandels -> users (user_id));
joinable!(marks -> mandels (mandela_id));
joinable!(marks -> users (user_id));
joinable!(users -> user_groups (group_id));
joinable!(votes -> mandels (mandela_id));
joinable!(votes -> users (user_id));

allow_tables_to_appear_in_same_query!(
    categories,
    comments,
    forum_categories,
    forum_sections,
    forum_topics,
    mandels,
    marks,
    user_groups,
    users,
    values,
    votes,
);
