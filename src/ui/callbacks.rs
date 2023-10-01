use crate::ui::{self, Operation};
use glib::clone;
use gtk::{glib, prelude::*, Button, FileChooserAction, MessageType, ResponseType};
use std::{cell::Cell, process::Command, rc::Rc};

pub fn open_file_chooser<W: IsA<gtk::Window>>(
    button: &Button,
    parent: Rc<W>,
    file: Rc<Cell<String>>,
    action: FileChooserAction,
    filters: &[&str],
) {
    let title = format!("Select {}", button.label().unwrap());

    let file_chooser = ui::build_file_chooser(title, parent, action);
    let filter = ui::build_file_filter(filters);

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

pub fn call_xdelta<W: IsA<gtk::Window>>(
    parent: Rc<W>,
    source: Rc<Cell<String>>,
    target: Rc<Cell<String>>,
    output: Rc<Cell<String>>,
    operation: &Operation,
) {
    let source = &source.take();
    let target = &target.take();
    let output = &output.take();

    let args = if operation == &Operation::Apply {
        ["-dfs", &source, &target, &output]
    } else {
        ["-efs", &source, &output, &target]
    };

    let output = Command::new("xdelta3")
        .args(args)
        .output()
        .expect("Unable to run 'xdelta3 {args}'");

    let message_type;
    let message;

    if output.status.success() {
        message_type = MessageType::Info;

        message = if operation == &Operation::Apply {
            "ROM successfully patched!".to_string()
        } else {
            "Patch successfully created!".to_string()
        }
    } else {
        message_type = MessageType::Error;
        let stderr = String::from_utf8(output.stderr).unwrap().trim().to_string();

        message = if stderr.contains("empty string") {
            format!(
                "Please select all of the required files, \nxdelta3 error: {:?}",
                stderr
            )
        } else {
            format!("An error has occurred, \nxdelta3 error: {:?}", stderr)
        }
    }

    let dialog = ui::build_dialog(parent, message_type, &message);
    dialog.connect_response(|obj, _| {
        obj.destroy();
    });
    dialog.show();
}
