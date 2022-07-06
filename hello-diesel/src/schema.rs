table! {
    casbin_rule (id) {
        id -> Integer,
        ptype -> Varchar,
        v0 -> Varchar,
        v1 -> Varchar,
        v2 -> Varchar,
        v3 -> Varchar,
        v4 -> Varchar,
        v5 -> Varchar,
    }
}

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

allow_tables_to_appear_in_same_query!(
    casbin_rule,
    posts,
    short_links,
);
