table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hashpass -> Text,
        salt -> Text,
        created -> Timestamp,
        stopped -> Timestamp,
        attempts -> Int4,
    }
}
