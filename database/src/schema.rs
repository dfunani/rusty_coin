// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        user_id -> Text,
        email -> Text,
        password -> Text,
        status -> Text,
        role -> Text,
        salt_value -> Text,
    }
}
