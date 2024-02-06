// @generated automatically by Diesel CLI.

diesel::table! {
    users (uid) {
        uid -> Int4,
        groups -> Nullable<Array<Nullable<Int4>>>,
    }
}
