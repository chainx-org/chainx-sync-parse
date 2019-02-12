table! {
    blocks (height, prefix) {
        height -> Int8,
        prefix -> Varchar,
        value -> Jsonb,
    }
}
