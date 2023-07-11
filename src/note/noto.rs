use chrono::{DateTime, Utc};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::{fmt, fs::File, io::BufReader};

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
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
    pub creation_date: DateTime<Utc>,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_vaulted: bool,
    pub access_date: DateTime<Utc>,
    pub scrolling_position: i64,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoFolder {
    pub id: i64,
    pub title: String,
    pub position: i64,
    pub color: NotoColor,
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
    pub filtering_type: String,
    pub open_notes_in: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoLabel {
    pub id: i64,
    pub folder_id: i64,
    pub title: String,
    pub color: NotoColor,
    pub position: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoNoteLabel {}

#[derive(Deserialize, Serialize)]
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
    pub is_show_notes_count: bool,
    pub is_do_not_disturb: bool,
    pub is_screen_on: bool,
    pub main_interface_id: i64,
    pub is_remember_scrolling_position: bool,
    pub all_notes_scrolling_position: i64,
    pub recent_notes_scrolling_position: i64,
    pub scheduled_notes_scrolling_position: i64,
    pub archived_notes_scrolling_position: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotoData {
    pub folders: Vec<NotoFolder>,
    pub notes: Vec<NotoNote>,
    pub labels: Vec<NotoLabel>,
    pub note_labels: Vec<NotoNoteLabel>,
    pub settings: NotoSettings,
}

impl fmt::Display for NotoFolder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

/// Reads a Noto backup into a struct
pub fn deserialize_noto_backup() -> NotoData {
    // TODO: prompt which noto backup to deserialize
    let file_path = "./data/noto/Noto.json";

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening Noto.json: {:?}", e),
    };

    let reader = BufReader::new(file);
    let data: NotoData = match serde_json::from_reader(reader) {
        Ok(noto_data) => noto_data,
        Err(e) => panic!("Unexpected JSON object: {:?}", e),
    };

    data
}

/// Creates an updated Noto backup
/// 
/// # Arguments
/// * `data` - noto data to be serialized
pub fn serialize_noto_data(data: &NotoData) {
    let mut file = match File::create("./data/noto/Noto.updated.json") {
        Ok(file) => file,
        Err(e) => panic!("Unable to open file for writing: {:?}", e)
    };

    match serde_json::to_writer_pretty(&mut file, data) {
        Ok(()) => (),
        Err(e) => panic!("Failed to write to JSON file: {:?}", e)
    }
}

/// Displays a prompt of available noto folder choices
/// 
/// # Arguments
/// * folders - list of noto folders
pub fn prompt_folder_selection(folders: &Vec<NotoFolder>) -> i64 {
    let folder_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a folder:")
        .item("Root (default)")
        .items(&folders[1..folders.len()])
        .default(0)
        .interact();

    let folder = match folder_selection {
        Ok(index) => index,
        Err(e) => panic!("Error selecting folder: {:?}", e)
    };

    folders[folder].id
}
