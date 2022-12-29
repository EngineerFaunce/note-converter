use crate::note::{KeepNote, NotoNote};
use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::{Tz, US::Eastern};
use core::panic;
use std::{fs, path::PathBuf};

mod note;

fn read_file(file_name: PathBuf) -> KeepNote {
    {
        let text = match fs::read_to_string(file_name) {
            Ok(json) => json,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        // todo: avoid unwrap()
        serde_json::from_str::<KeepNote>(&text).unwrap()
    }
}

fn read_notes() -> Vec<KeepNote> {
    let mut notes_arr: Vec<KeepNote> = Vec::new();

    let keep_notes = match fs::read_dir("./src/keep-notes/") {
        Ok(dir) => dir,
        Err(e) => panic!("Error reading directory: {:?}", e),
    };

    for file in keep_notes {
        let data = match file {
            Ok(entry) => read_file(entry.path()),
            Err(e) => panic!("Error reading file: {:?}", e),
        };

        notes_arr.push(data);
    }

    notes_arr
}

fn main() {
    let serialized_notes = read_notes();

    let mut converted_notes: Vec<NotoNote> = Vec::new();

    for note in serialized_notes {
        println!("Original note title: {}", note.title);

        let eastern_time: DateTime<Tz>;

        if let Some(time) =
            NaiveDateTime::from_timestamp_opt((note.created_timestamp_usec / 1000000) as i64, 0)
        {
            eastern_time = Eastern.from_utc_datetime(&time);
        } else {
            panic!("Error reading time from keep note.")
        }

        let noto_note: NotoNote = NotoNote {
            id: 1,
            folder_id: -1,
            title: eastern_time.format("%Y-%m-%d").to_string(),
            body: note.text_content,
            position: 0,
            creation_date: eastern_time.naive_utc(),
            is_pinned: false,
            is_archived: note.is_archived,
            is_vaulted: false,
            access_date: eastern_time.naive_utc(),
            scrolling_position: 0,
        };


        converted_notes.push(noto_note);
    }

    converted_notes.sort_by(|a, b| a.creation_date.cmp(&b.creation_date));

    for note in &converted_notes {
        println!(
            "New title: {}\n\"Creation date\": {}\n",
            note.title, note.creation_date
        );
    }

    println!("Number of converted notes: {}", converted_notes.len());

}
