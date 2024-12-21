// @generated automatically by Diesel CLI.
diesel print-schema > src/schema.rs

diesel::table! {
    permissions (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    roles (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        role_id -> Nullable<Integer>,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    roles,
    users,
);
