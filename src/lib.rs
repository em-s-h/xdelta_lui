use gtk::{prelude::*, FileChooserNative, FileChooserAction};
use std::process::Command;

#[derive(Default)]
pub struct Files {
    source: String,
    target: String,
    output: String,
}

// #[derive(Debug)]
// pub enum Mode {
//     Create,
//     Apply,
// }

const BUFFER_SIZE: u32 = 1000000000;

fn call_xdelta(args: &[&str]) {
    Command::new("xdelta3").args(args).spawn()
        .expect("Unable to run 'xdelta3 {args}'");
}

fn choose_file(title: &str, action: FileChooserAction, parent: &impl IsA<gtk::Window>) {
    let parent = Option::from(parent);
    let title = Option::from(title);

    let file_chooser = FileChooserNative::new(title, parent, action, None, None);
}

fn build_progress_window(message: &str) {
    println!("{:?}", message);
}

fn get_file_name(path: &str) -> &str {
    unimplemented!()
}

#[cfg(test)]
mod test {
}
