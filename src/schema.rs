table! {
    languages (language_id) {
        language_id -> Int4,
        priority -> Int4,
        name -> Text,
        highlighter_mode -> Nullable<Text>,
        mime -> Text,
        is_markdown -> Bool,
        identifier -> Text,
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

table! {
    wrappers (wrapper_id) {
        wrapper_id -> Int4,
        language_id -> Int4,
        label -> Text,
        code -> Text,
        ordering -> Int4,
        is_formatter -> Bool,
        is_asm -> Bool,
        identifier -> Text,
    }
}

joinable!(pastes -> languages (language_id));
joinable!(wrappers -> languages (language_id));

allow_tables_to_appear_in_same_query!(languages, pastes, wrappers,);
