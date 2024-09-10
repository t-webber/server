#![allow(
    clippy::pub_use,
    clippy::missing_trait_methods,
    clippy::single_char_lifetime_names
)]

// @generated automatically by Diesel CLI.
diesel::table! {
    notes (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        updated_at -> Timestamp,
    }
}
