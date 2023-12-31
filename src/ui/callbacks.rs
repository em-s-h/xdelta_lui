use crate::ui::Operation;
use gio::{Cancellable, File};
use glib::{clone, Error};
use gtk::{gio, glib, prelude::*, AlertDialog, Button, FileChooserAction, FileDialog, FileFilter};
use std::{cell::Cell, process::Command, rc::Rc};

pub fn open_file_chooser<W: IsA<gtk::Window>>(
    parent: Rc<W>,
    button: &Button,
    file: Rc<Cell<String>>,
    action: FileChooserAction,
    patterns: &[&str],
) {
    // {{{
    let label = {
        let mut lbl = button.label().unwrap().to_string();
        let start = lbl.find(":").unwrap_or(0) + 1;
        lbl.replace_range(start.., " ");
        lbl
    };

    let title = format!("Select {}", label);
    let filter = {
        // Build file filter {{{
        let filter = FileFilter::new();

        for pat in patterns.iter() {
            filter.add_pattern(pat);
        }
        filter
    };
    // }}}

    let file_dialog = FileDialog::builder()
        .title(title)
        .default_filter(&filter)
        .modal(true)
        .build();

    let callback = clone!(@strong file, @strong button => move |r: Result<File, Error>| {
        if let Ok(f) = r {
            button.set_label(
                &format!("{} \"{}\"",
                    label,
                    f.basename()
                        .expect("Unable to obtain file basename")
                        .to_str()
                        .expect("Unable to convert basename to &str")
                )
            );

            file.set(
                f.path()
                    .expect("Unable to obtain file path")
                    .to_str()
                    .expect("Unable to convert path to &str")
                    .to_string(),
            );
        }
    });

    if action == FileChooserAction::Open {
        file_dialog.open(Some(&*parent), Some(&Cancellable::new()), callback);
        //
    } else if action == FileChooserAction::Save {
        file_dialog.save(Some(&*parent), Some(&Cancellable::new()), callback);
    }
}
// }}}

pub fn call_xdelta<W: IsA<gtk::Window>>(
    parent: Rc<W>,
    source: Rc<Cell<String>>,
    target: Rc<Cell<String>>,
    output: Rc<Cell<String>>,
    operation: Operation,
) {
    // {{{
    let s_file = source.take();
    let t_file = target.take();
    let o_file = output.take();

    let args = if operation == Operation::Apply {
        ["-dfs", &s_file, &t_file, &o_file]
    } else {
        ["-efs", &s_file, &o_file, &t_file]
    };

    let out = Command::new("xdelta3")
        .args(args)
        .output()
        .expect("Unable to run 'xdelta3 {args}'");

    let details;
    let message;

    if out.status.success() {
        // Alert dialog details {{{
        message = "Success!";

        details = if operation == Operation::Apply {
            "ROM successfully patched!".to_string()
        } else {
            "Patch successfully created!".to_string()
        };
    } else {
        message = "Error!";
        let stderr = String::from_utf8(out.stderr).unwrap().trim().to_string();

        details = if stderr.contains("empty string") {
            format!(
                "Please select all of the required files, \nxdelta3 error: {:?}",
                stderr
            )
        } else {
            format!("An error has occurred, \nxdelta3 error: {:?}", stderr)
        }
    }
    // }}}

    let dialog = AlertDialog::builder()
        .detail(details)
        .message(message)
        .modal(true)
        .build();

    dialog.show(Some(&*parent));
}
// }}}
