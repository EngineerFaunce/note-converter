use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoNote {
    pub id: i64,
    pub folder_id: i64,
    pub title: String,
    pub body: String,
    pub position: i32,
    pub creation_date: NaiveDateTime,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_vaulted: bool,
    pub access_date: NaiveDateTime,
    pub scrolling_position: i64,
}
