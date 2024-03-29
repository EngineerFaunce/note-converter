use dialoguer::{theme::ColorfulTheme, Select};
use note::{convert_notes, NoteFormat};

mod note;

fn main() {
    let source_formats = vec![NoteFormat::GoogleKeep, NoteFormat::Noto];
    let target_formats = vec![NoteFormat::Noto, NoteFormat::Markdown];

    let source_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the format of the source notes:")
        .items(&source_formats)
        .default(0)
        .interact();

    let source = match source_selection {
        Ok(index) => &source_formats[index],
        _ => panic!("Invalid selection"),
    };

    let filtered_formats = &target_formats.iter().filter(|&format| format != source).collect::<Vec<_>>();

    let target_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the format of the converted notes:")
        .items(&filtered_formats)
        .default(0)
        .interact();

    let target = match target_selection {
        Ok(index) => &filtered_formats[index],
        _ => panic!("Invalid selection"),
    };

    convert_notes(source, target);
}
