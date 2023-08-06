// @generated automatically by Diesel CLI.

diesel::table! {
    logs (id) {
        id -> Integer,
        user_agent -> Text,
        response_time -> Integer,
        timestamp -> Timestamp,
    }
}
