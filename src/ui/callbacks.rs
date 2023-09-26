use super::Mode;
use glib::clone;
use gtk::{glib, prelude::*, Button, FileChooserAction, ResponseType};
use std::{cell::Cell, process::Command, rc::Rc};

pub fn open_file_chooser<W: IsA<gtk::Window>>(
    button: &Button,
    parent: Rc<W>,
    file: Rc<Cell<String>>,
    action: FileChooserAction,
    filters: &[&str],
) {
    let title = format!("Select {}", button.label().unwrap());

    let file_chooser = super::build_file_chooser(&title, parent, action);
    let filter = super::build_file_filter(filters);

    file_chooser.connect_response(clone!(@strong file, @strong button => move |obj, r| {
        if r == ResponseType::Accept {
            if let Some(f) = obj.file() {
                button.set_label(&f
                    .basename()
                    .expect("Unable to obtain file basename")
                    .to_str()
                    .expect("Unable to convert basename to &str"));

                file.set(f
                    .path()
                    .expect("Unable to obtain file path")
                    .to_str()
                    .expect("Unable to convert path to &str")
                    .to_string());
            }
        }
        obj.destroy();
    }));

    file_chooser.set_filter(&filter);
    file_chooser.show();
}

pub fn call_xdelta(
    source: Rc<Cell<String>>,
    target: Rc<Cell<String>>,
    output: Rc<Cell<String>>,
    mode: Mode,
) {
    let source = &source.take();
    let target = &target.take();
    let output = &output.take();

    let args = if mode == Mode::Apply {
        ["-d", "-s", &source, &target, &output]
    } else {
        ["-e", "-s", &source, &output, &target]
    };

    Command::new("xdelta3")
        .args(args)
        .spawn()
        .expect("Unable to run 'xdelta3 {args}'");
}
