table! {
    languages (language_id) {
        language_id -> Int4,
        priority -> Int4,
        name -> Text,
        highlighter_mode -> Nullable<Text>,
        mime -> Text,
        is_markdown -> Bool,
    }
}

table! {
    pastes (paste_id) {
        paste_id -> Int4,
        identifier -> Text,
        delete_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        language_id -> Int4,
        paste -> Text,
    }
}

joinable!(pastes -> languages (language_id));

allow_tables_to_appear_in_same_query!(languages, pastes,);
