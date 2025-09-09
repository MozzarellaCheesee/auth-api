diesel::table! {
    books (id) {
        id -> Int8,
        name -> Text,
        author -> Text,
        description -> Text,
        rating -> Nullable<Numeric>,
    }
}

diesel::table! {
    comments (book_id, id) {
        id -> Int8,
        book_id -> Int8,
        author_id -> Int8,
        text -> Text,
        rating -> Float8,
    }
}

diesel::table! {
    issued_jwt_tokens (jti) {
        #[max_length = 36]
        jti -> Varchar,
        user_id -> Int8,
        revoked -> Bool,
        #[max_length = 36]
        device_id -> Varchar,
    }
}

diesel::table! {
    user_library (user_id, book_id) {
        user_id -> Int8,
        book_id -> Int8,
        rating -> Nullable<Int2>,
        comment -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        first_name -> Text,
        second_name -> Text,
        #[max_length = 20]
        username -> Varchar,
        #[sql_name = "e-mail"]
        e_mail -> Text,
        #[max_length = 100]
        password -> Varchar,
        #[sql_name = "e-mail_confirmed"]
        e_mail_confirmed -> Bool,
        #[max_length = 15]
        role_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    users_data (id) {
        id -> Int8,
        #[max_length = 16]
        nickname -> Nullable<Varchar>,
        #[max_length = 250]
        description -> Nullable<Varchar>,
    }
}

diesel::joinable!(comments -> books (book_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(issued_jwt_tokens -> users (user_id));
diesel::joinable!(user_library -> books (book_id));
diesel::joinable!(user_library -> users (user_id));
diesel::joinable!(users_data -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    comments,
    issued_jwt_tokens,
    user_library,
    users,
    users_data,
);