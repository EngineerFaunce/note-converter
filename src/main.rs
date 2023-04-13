use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::{Tz, US::Eastern};
use core::panic;
use note::noto::NotoNote;

mod note;

fn main() {
    let source_notes = note::read_notes();

    let mut converted_notes: Vec<NotoNote> = Vec::new();

    for note in source_notes {
        let time = match NaiveDateTime::from_timestamp_opt(
            (note.created_timestamp_usec / 1000000) as i64,
            0,
        ) {
            Some(dt) => dt,
            None => panic!("Invalid timestamp."),
        };
        // convert the keep note timestamp into an ISO 8601 datetime
        let eastern_time: DateTime<Tz> = Eastern.from_utc_datetime(&time);

        // serialize the keep note data into noto format
        let noto_note: NotoNote = NotoNote {
            id: 1,
            folder_id: -1, // ! change once we are able to get folder selection from user
            title: eastern_time.format("%Y-%m-%d").to_string(),
            body: note.text_content,
            position: 0,
            creation_date: DateTime::from_utc(time, Utc),
            is_pinned: false,
            is_archived: note.is_archived,
            is_vaulted: false,
            access_date: DateTime::from_utc(time, Utc),
            scrolling_position: 0,
        };

        println!("Original note title: {}", note.title);
        println!(
            "New title: {}\n\"Creation date\": {}\n",
            noto_note.title, noto_note.creation_date
        );

        converted_notes.push(noto_note);
    }

    converted_notes.sort_by(|a, b| a.creation_date.cmp(&b.creation_date));

    println!("Number of notes to be converted: {}", converted_notes.len());

    let noto = note::deserialize_noto_backup();

    println!("{}", noto.folders[0].title);
}
