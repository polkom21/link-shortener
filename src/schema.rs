// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        #[max_length = 64]
        short -> Varchar,
        original -> Text,
        created -> Timestamptz,
    }
}
