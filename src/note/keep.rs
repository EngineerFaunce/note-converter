use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepNote {
    _color: String,
    _is_trashed: bool,
    _is_pinned: bool,
    pub is_archived: bool,
    pub text_content: String,
    pub title: String,
    pub user_edited_timestamp_usec: u64,
    pub created_timestamp_usec: u64,
}
