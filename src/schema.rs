// @generated automatically by Diesel CLI.

diesel::table! {
    user_hash (UserID) {
        UserID -> Unsigned<Integer>,
        PasswordHash -> Char,
    }
}

diesel::table! {
    user_sessions (SessionID) {
        UserID -> Unsigned<Integer>,
        SessionID -> Varchar,
        Expire -> Datetime,
    }
}

diesel::table! {
    users (UserID) {
        UserID -> Unsigned<Integer>,
        UserName -> Varchar,
        Email -> Varchar,
        IsVerified -> Bool,
        Created -> Datetime,
    }
}

diesel::joinable!(user_hash -> users (UserID));
diesel::joinable!(user_sessions -> users (UserID));

diesel::allow_tables_to_appear_in_same_query!(
    user_hash,
    user_sessions,
    users,
);
