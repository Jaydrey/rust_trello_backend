// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 20]
        phone_number -> Varchar,
        password -> Text,
        #[max_length = 20]
        role -> Varchar,
        date_of_birth -> Nullable<Text>,
        active -> Bool,
        created_at -> Text,
        updated_at -> Nullable<Text>,
    }
}
