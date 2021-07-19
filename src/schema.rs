table! {
    config (id) {
        id -> Integer,
        fill_version -> Text,
        asset_regex -> Text,
        github_user -> Nullable<Text>,
        github_repo -> Nullable<Text>,
    }
}

table! {
    target_version_mappings (target_id, current_version) {
        target_id -> Integer,
        current_version -> Text,
        update_version -> Text,
    }
}

table! {
    targets (id) {
        id -> Integer,
        target -> Text,
        regex -> Text,
    }
}

joinable!(target_version_mappings -> targets (target_id));

allow_tables_to_appear_in_same_query!(config, target_version_mappings, targets,);
