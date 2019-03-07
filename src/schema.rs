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
    paste_contents (paste_content_id) {
        paste_content_id -> Int4,
        paste_revision_id -> Int4,
        language_id -> Int4,
        paste -> Text,
    }
}

table! {
    paste_revisions (paste_revision_id) {
        paste_revision_id -> Int4,
        created_at -> Timestamptz,
        paste_id -> Int4,
    }
}

table! {
    pastes (paste_id) {
        paste_id -> Int4,
        identifier -> Text,
        delete_at -> Nullable<Timestamptz>,
    }
}

joinable!(paste_contents -> languages (language_id));
joinable!(paste_contents -> paste_revisions (paste_revision_id));
joinable!(paste_revisions -> pastes (paste_id));

allow_tables_to_appear_in_same_query!(languages, paste_contents, paste_revisions, pastes,);
