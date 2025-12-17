diesel::table! {
    users (id) {
        id -> Bigint,
        user_name -> Varchar,
        password -> Varchar,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
        unregistered -> Integer
    }
}
