table! {
    posts (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    short_links (id) {
        id -> Unsigned<Integer>,
        url -> Varchar,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    short_links,
    users,
);
