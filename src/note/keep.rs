use std::{
    fs::{self, File},
    io::BufReader,
};

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

/// Reads a directory of Google Keep notes into an array
pub fn read_notes() -> Vec<KeepNote> {
    // get keep notes directory
    let mut files = Vec::new();
    let keep_notes_dir = match fs::read_dir("./data/keep-notes/") {
        Ok(dir) => dir,
        Err(e) => panic!("Error reading directory: {:?}", e),
    };

    // read file paths in keep notes directory into array
    for file in keep_notes_dir {
        match file {
            Ok(file) => {
                let path = file.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    files.push(path);
                }
            }
            Err(e) => panic!("Error reading file: {:?}", e),
        };
    }

    // read each file's contents into an array
    let mut notes: Vec<KeepNote> = Vec::new();
    for file_path in files {
        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file: {:?}", e),
        };
        let reader = BufReader::new(file);

        let keep_note: KeepNote = match serde_json::from_reader(reader) {
            Ok(keep_note) => keep_note,
            Err(e) => panic!("Unexpected JSON object: {:?}", e),
        };

        notes.push(keep_note);
    }

    notes.sort_by(|a, b| a.created_timestamp_usec.cmp(&b.created_timestamp_usec));

    notes
}
