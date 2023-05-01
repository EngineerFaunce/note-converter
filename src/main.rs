use crate::note::{noto::deserialize_noto_backup, keep::read_notes};

mod note;

fn main() {
    let noto = deserialize_noto_backup();

    let source_notes = read_notes();
}
