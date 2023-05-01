use crate::note::{noto::{deserialize_noto_backup, prompt_folder_selection}, keep::read_notes};

mod note;

fn main() {
    let noto = deserialize_noto_backup();

    let chosen_folder_id = prompt_folder_selection(&noto.folders);

    let source_notes = read_notes();
}
