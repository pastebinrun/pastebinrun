table! {
    implementations (implementation_id) {
        implementation_id -> Int4,
        language_id -> Int4,
        identifier -> Text,
        label -> Text,
    }
}

table! {
    implementation_wrappers (implementation_wrapper_id) {
        implementation_wrapper_id -> Int4,
        implementation_id -> Int4,
        identifier -> Text,
        label -> Text,
        code -> Text,
        ordering -> Int4,
        is_formatter -> Bool,
        is_asm -> Bool,
    }
}

table! {
    languages (language_id) {
        language_id -> Int4,
        priority -> Int4,
        name -> Text,
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

joinable!(implementation_wrappers -> implementations (implementation_id));
joinable!(implementations -> languages (language_id));
joinable!(pastes -> languages (language_id));

allow_tables_to_appear_in_same_query!(implementations, implementation_wrappers, languages, pastes,);
