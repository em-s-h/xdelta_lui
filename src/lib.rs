use gtk::{prelude::*, FileChooserAction, FileChooserNative, FileFilter};
use std::process::Command;

#[derive(Default, Debug)]
pub struct Files {
    pub source: String,
    pub target: String,
    pub output: String,
}

// #[derive(Debug)]
// pub enum Mode {
//     Create,
//     Apply,
// }

const BUFFER_SIZE: u32 = 1000000000;

pub fn call_xdelta(args: &[&str]) {
    Command::new("xdelta3")
        .args(args)
        .spawn()
        .expect("Unable to run 'xdelta3 {args}'");
}

pub fn build_file_chooser(
    title: &str,
    action: FileChooserAction,
    parent: &impl IsA<gtk::Window>,
    filter: &FileFilter,
) -> FileChooserNative {
    let accept_label = if action == FileChooserAction::Open {
        "Open".to_string()
    } else if action == FileChooserAction::Save {
        "Save".to_string()
    } else {
        "Select".to_string()
    };

    FileChooserNative::builder()
        .title(title)
        .action(action)
        .transient_for(parent)
        .filter(&filter)
        .visible(true)
        .accept_label(accept_label)
        .build()
}

pub fn build_file_filter(patterns: &[&str]) -> FileFilter {
    let filter = FileFilter::new();

    for pat in patterns.iter() {
        filter.add_pattern(pat);
    }

    filter
}

// fn build_progress_window(message: &str) {
//     println!("{:?}", message);
// }

// fn get_file_name(path: &str) -> &str {
//     unimplemented!()
// }

#[cfg(test)]
mod test {}
