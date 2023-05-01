use crate::note::{keep, noto};

mod note;

fn main() {
    let noto = noto::deserialize_noto_backup();

    let chosen_folder_id = noto::prompt_folder_selection(&noto.folders);
    let max_id = noto.notes[0].id;

    let source_notes = keep::read_notes();

    let converted_notes = note::convert_notes(source_notes, chosen_folder_id, max_id);

}
