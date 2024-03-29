use core::panic;
use std::{fmt, fs::File, io::Write};

use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use dialoguer::{theme::ColorfulTheme, Select};

use self::{
    keep::KeepNote,
    noto::{NotoData, NotoNote},
};

pub mod keep;
pub mod noto;

/// Supported note formats
#[derive(PartialEq)]
pub enum NoteFormat {
    GoogleKeep,
    Noto,
    Markdown,
}

impl fmt::Display for NoteFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoteFormat::GoogleKeep => write!(f, "Google Keep"),
            NoteFormat::Markdown => write!(f, "Markdown"),
            NoteFormat::Noto => write!(f, "Noto"),
        }
    }
}

#[derive(Clone)]
enum FileNameFormat {
    Original,
    Date,
}

impl fmt::Display for FileNameFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileNameFormat::Original => write!(f, "Use note's original title"),
            FileNameFormat::Date => write!(f, "Use the note's creation date (YYYY-MM-DD)"),
        }
    }
}

/// Returns an array of converted notes
///
/// # Arguments
/// * `source` - note format of the notes to be converted
/// * `target` - note format for the converted notes
pub fn convert_notes(source: &NoteFormat, target: &NoteFormat) {
    match (source, target) {
        (NoteFormat::GoogleKeep, NoteFormat::Noto) => {
            let source_notes = keep::read_notes();
            let noto = noto::deserialize_noto_backup();

            convert_keep_to_noto(source_notes, noto);
        }
        (NoteFormat::GoogleKeep, NoteFormat::Markdown) => {
            let source_notes = keep::read_notes();
            convert_keep_to_markdown(&source_notes);
        }
        (NoteFormat::Noto, NoteFormat::Markdown) => {
            let noto = noto::deserialize_noto_backup();
            convert_noto_to_markdown(&noto);
        }
        _ => {
            panic!("error")
        }
    }
}

fn convert_usec_timestamp_to_datetime(timestamp: u64) -> NaiveDateTime {
    let time = match NaiveDateTime::from_timestamp_opt((timestamp / 1000000) as i64, 0) {
        Some(dt) => dt,
        None => panic!("Invalid timestamp."),
    };

    time
}

fn prompt_title_format() -> FileNameFormat {
    let format_options = vec![FileNameFormat::Original, FileNameFormat::Date];

    let format_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the converted note's file name format: ")
        .items(&format_options)
        .default(0)
        .interact();

    let format = match format_selection {
        Ok(index) => index,
        Err(e) => panic!("Error selecting folder: {:?}", e),
    };

    format_options[format].clone()
}

/// Converts google keep notes to the Noto format
///
/// # Arguments
/// * `source_notes` - Vec of google keep notes
/// * `noto` - NotoData struct
fn convert_keep_to_noto(source_notes: Vec<KeepNote>, mut noto: NotoData) {
    let mut converted_notes: Vec<NotoNote> = Vec::new();

    // Get folder ID
    let chosen_folder_id = noto::prompt_folder_selection(&noto.folders);

    let title_format: FileNameFormat = prompt_title_format();

    // max ID will always be the most recent note
    let mut note_id = noto.notes[0].id;

    // the max position of notes within a folder will be the first note we find
    let mut note_position = 0;
    for note in &noto.notes {
        if note.folder_id == chosen_folder_id {
            note_position = note.position;
            break;
        }
    }

    for note in source_notes {
        note_id += 1;
        note_position += 1;

        // convert the keep note timestamp into an ISO 8601 datetime
        let time = convert_usec_timestamp_to_datetime(note.created_timestamp_usec);
        let user_timezone = Local::now().timezone();
        let user_time = user_timezone.from_utc_datetime(&time);

        // convert the keep note data into noto format
        let noto_note: NotoNote = NotoNote {
            id: note_id,
            folder_id: chosen_folder_id,
            title: match title_format {
                FileNameFormat::Original => note.title,
                FileNameFormat::Date => user_time.format("%Y-%m-%d").to_string(),
            },
            body: note.text_content,
            position: note_position,
            creation_date: DateTime::from_utc(time, Utc),
            is_pinned: false,
            is_archived: note.is_archived,
            is_vaulted: false,
            access_date: DateTime::from_utc(time, Utc),
            scrolling_position: 0,
        };

        converted_notes.push(noto_note);
    }

    // add converted_notes to noto backup
    noto.notes.append(&mut converted_notes);

    noto::serialize_noto_data(&noto);
}

/// Converts google keep notes to markdown files
fn convert_keep_to_markdown(source_notes: &Vec<KeepNote>) {
    let title_format: FileNameFormat = prompt_title_format();

    for note in source_notes {
        // convert the keep note timestamp into an ISO 8601 datetime
        let time = convert_usec_timestamp_to_datetime(note.created_timestamp_usec);
        let user_timezone = Local::now().timezone();
        let user_time = user_timezone.from_utc_datetime(&time);

        let file_name = match title_format {
            FileNameFormat::Original => note.title.replace("/", "_"), // remove any slashes from the note title
            FileNameFormat::Date => user_time.format("%Y-%m-%d").to_string(),
        };

        let file_path = format!("./data/markdown/{}.md", file_name);

        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(e) => panic!("Unable to create markdown file for {}: {:?}", note.title, e),
        };

        match file.write_all(note.text_content.as_bytes()) {
            Ok(()) => (),
            Err(e) => panic!("Error writing keep note text content to file: {:?}", e),
        }
    }
}

/// Converts Noto notes to markdown files
fn convert_noto_to_markdown(backup_data: &NotoData) {
    // Get folder ID
    let chosen_folder_id = noto::prompt_folder_selection(&backup_data.folders);

    let title_format: FileNameFormat = prompt_title_format();

    for note in &backup_data.notes {
        if note.folder_id == chosen_folder_id {
            let file_name = match title_format {
                FileNameFormat::Original => note.title.replace("/", "_"), // remove any slashes from the note title
                FileNameFormat::Date => note.creation_date.format("%Y-%m-%d").to_string(),
            };
            let file_path = format!("./data/markdown/{}.md", file_name);

            let mut file = match File::create(file_path) {
                Ok(file) => file,
                Err(e) => panic!("Unable to create markdown file for {}: {:?}", note.title, e),
            };

            match file.write_all(note.body.as_bytes()) {
                Ok(()) => (),
                Err(e) => panic!("Error writing note text content to file: {:?}", e),
            }
        }
    };
}
