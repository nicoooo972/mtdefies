// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        pseudo -> Text,
        pw -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
