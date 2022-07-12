table! {
    sessionsinfo (id) {
        id -> Int4,
        sessionid -> Text,
        userid -> Int4,
        expire -> Timestamp,
    }
}

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

allow_tables_to_appear_in_same_query!(sessionsinfo, users,);
