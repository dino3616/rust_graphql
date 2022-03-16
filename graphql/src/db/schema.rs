table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        profile -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
