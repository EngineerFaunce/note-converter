use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub enum NotoColor {
    Gray,
    Blue,
    Pink,
    Cyan,
    Purple,
    Red,
    Yellow,
    Orange,
    Green,
    Brown,
    BlueGray,
    Teal,
    Indigo,
    DeepPurple,
    DeepOrange,
    DeepGreen,
    LightBlue,
    LightGreen,
    LightRed,
    LightPink,
    Black,
}

#[derive(Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoNote {
    pub id: i64,
    pub folder_id: i64,
    pub title: String,
    pub body: String,
    pub position: i32,
    #[serde(with = "ts_seconds")]
    pub creation_date: DateTime<Utc>,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_vaulted: bool,
    #[serde(with = "ts_seconds")]
    pub access_date: DateTime<Utc>,
    pub scrolling_position: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoFolder {
    pub id: i64,
    pub title: String,
    pub position: i64,
    pub color: NotoColor,
    #[serde(with = "ts_seconds")]
    pub creation_date: DateTime<Utc>,
    pub layout: String,
    pub note_preview_size: i32,
    pub is_archived: bool,
    pub is_pinned: bool,
    pub is_show_note_creation_date: bool,
    pub new_note_cursor_position: String,
    pub sorting_type: String,
    pub sorting_order: SortOrder,
    pub grouping: String,
    pub grouping_order: String,
    pub is_vaulted: bool,
    pub scrolling_position: i64,
    pub filterting_type: String,
    pub open_note_in: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoLabel {
    pub id: i64,
    pub folder_id: i64,
    pub title: String,
    pub color: NotoColor,
    pub position: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoNoteLabel {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoSettings {
    pub theme: String,
    pub font: String,
    pub language: String,
    pub icon: String,
    pub vault_timeout: String,
    pub is_vault_open: bool,
    pub is_bio_auth_enabled: bool,
    pub last_version: String,
    pub sorting_type: String,
    pub sorting_order: SortOrder,
    pub is_show_notes_counts: bool,
    pub is_do_not_disturb: bool,
    pub is_screen_on: bool,
    pub main_interface_id: i64,
    pub is_remember_scrolling_position: bool,
    pub all_notes_scrolling_position: i64,
    pub recent_notes_scrolling_position: i64,
    pub scheduled_notes_scrolling_position: i64,
    pub archived_notes_scrolling_position: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoData {
    pub folders: Vec<NotoFolder>,
    pub notes: Vec<NotoNote>,
    pub labels: Vec<NotoLabel>,
    pub note_labels: Vec<NotoNoteLabel>,
    pub settings: NotoSettings,
}
