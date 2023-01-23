table! {
    metadata (hash) {
        hash -> Text,
        name -> Text,
        path -> Text,
        contentType -> Text,
        status -> Text,
        timestampCreated -> Integer,
        timestampModified -> Integer,
        extension -> Nullable<Text>,
        tags -> Nullable<Text>,
        notes -> Nullable<Text>,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        duration -> Nullable<Integer>,
    }
}

table! {
    preferences (key) {
        key -> Text,
        value -> Nullable<Text>,
    }
}

table! {
    smart_folder (path) {
        name -> Text,
        path -> Text,
        numberOfFiles -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(metadata, preferences, smart_folder,);
