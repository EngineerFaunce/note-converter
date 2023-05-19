use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::{Tz, US::Eastern};

use self::{keep::KeepNote, noto::NotoNote};

pub mod keep;
pub mod noto;

pub fn convert_notes(keep_notes: Vec<KeepNote>, folder_id: i64, max_id: i64, max_position: i32) -> Vec<NotoNote> {
    let mut converted_notes: Vec<NotoNote> = Vec::new();
    let mut note_id = max_id;
    let mut note_position = max_position;

    for note in keep_notes {
        note_id += 1;
        note_position += 1;

        let time = match NaiveDateTime::from_timestamp_opt(
            (note.created_timestamp_usec / 1000000) as i64,
            0,
        ) {
            Some(dt) => dt,
            None => panic!("Invalid timestamp."),
        };
        // convert the keep note timestamp into an ISO 8601 datetime
        // TODO: need to determine user's timezone
        let eastern_time: DateTime<Tz> = Eastern.from_utc_datetime(&time);

        // serialize the keep note data into noto format
        let noto_note: NotoNote = NotoNote {
            id: note_id,
            folder_id,
            title: eastern_time.format("%Y-%m-%d").to_string(),
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

    converted_notes
}
