table! {
    achievements (id) {
        id -> Int4,
        body -> Text,
        duration -> Int4,
    }
}

table! {
    facts (id) {
        id -> Int4,
        title -> Nullable<Text>,
        body -> Text,
        link -> Nullable<Text>,
    }
}

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

allow_tables_to_appear_in_same_query!(achievements, facts, sessionsinfo, users,);
