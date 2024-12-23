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
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    accounts (id) {
        id -> Text,
        account_id -> Text,
        user_id -> Text,
        status -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    payments (id) {
        id -> Text,
        payment_id -> Text,
        account_id -> Text,
        card_id -> Text,
        name -> Text,
        description -> Text,
        status -> Text,
        balance -> Float8,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    profiles (id) {
        id -> Text,
        profile_id -> Text,
        account_id -> Text,
        first_name -> Text,
        last_name -> Text,
        username -> Text,
        date_of_birth -> Date,
        gender -> Text,
        profile_picture -> Text,
        mobile_number -> Text,
        country -> Text,
        language -> Text,
        biography -> Text,
        occupation -> Text,
        interests -> Text,
        social_media_links -> Text,
        status -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    settings (id) {
        id -> Text,
        settings_id -> Text,
        account_id -> Text,
        email_status -> Text,
        communication_status -> Text,
        mfa_enabled -> Text,
        mfa_last_used_date -> Timestamp,
        profile_visibility_preference -> Text,
        data_sharing_preferences -> Text,
        communication_preference -> Text,
        location_tracking_enabled -> Bool,
        cookies_enabled -> Bool,
        theme_preference -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
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
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    cards (id) {
        id -> Text,
        card_id -> Text,
        card_number -> Text,
        card_type -> Text,
        status -> Text,
        pin -> Text,
        salt_value -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    transactions (id){
        id -> Text,
        transaction_id -> Text,
        sender -> Text,
        receiver -> Text,
        amount -> Float8,
        title -> Text,
        description -> Text,
        sender_signiture -> Text,
        receiver_signiture -> Text,
        transaction_status -> Text,
        salt_value -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    contracts (id){
        id -> Text,
        contract_id -> Text,
        contractor -> Text,
        contractee -> Text,
        title -> Text,
        description -> Text,
        contract -> Text,
        contract_status -> Text,
        contractor_signiture -> Text,
        contractee_signiture -> Text,
        salt_value -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}

diesel::table! {
    blocks(id) {
        id -> Text,
        block_id -> Text,
        transaction_id -> Text,
        contract_id -> Text,
        previous_block_id -> Text,
        next_block_id -> Text,
        block_type -> Text,
        created_date -> Timestamp,
        updated_date -> Timestamp,
    }
}
