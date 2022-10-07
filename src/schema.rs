// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        provider -> Nullable<Text>,
        provider_account_id -> Nullable<Text>,
        refresh_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        expires_at -> Nullable<Int4>,
        token_type -> Nullable<Text>,
        scope -> Nullable<Text>,
        id_token -> Nullable<Text>,
        session_state -> Nullable<Text>,
        oauth_token_secret -> Nullable<Text>,
        oauth_token -> Nullable<Text>,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

diesel::table! {
    session (id) {
        id -> Uuid,
        expires -> Timestamp,
        session_token -> Text,
        user_id -> Uuid,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        name -> Text,
        email_verified -> Nullable<Timestamp>,
    }
}

diesel::table! {
    verification_token (id) {
        id -> Uuid,
        identifier -> Text,
        token -> Text,
        expires -> Timestamp,
    }
}

diesel::joinable!(account -> user (user_id));
diesel::joinable!(session -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    recipes,
    schema_migrations,
    session,
    user,
    verification_token,
);
