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

diesel::table! {
    login_histories (id) {
        id -> Text,
        login_id -> Text,
        user_id -> Text,
        session_id -> Text,
        login_date -> Timestamp,
        login_location -> Text,
        login_device -> Text,
        login_method -> Text,
        logged_in -> Bool,
        logout_date -> Timestamp,
        authentication_token -> Text,
    }
}
