// @generated automatically by Diesel CLI.

diesel::table! {
    posts (uid) {
        uid -> Int4,
        groups -> Nullable<Array<Nullable<Int4>>>,
    }
}
