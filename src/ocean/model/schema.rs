table! {
    topics (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        create_ts -> Timestamptz,
        update_ts -> Timestamptz,
    }
}
