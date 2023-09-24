use crate::lib::Mode;
use glib::clone;
use gtk::{
    glib, prelude::*, ApplicationWindow, Box, Button, FileChooserAction, FileChooserDialog,
    FileChooserNative, FileFilter, Label, Notebook, Orientation, ResponseType,
};
use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    ops::DerefMut,
    rc::Rc,
    sync::{Arc, Mutex},
};

mod callbacks;

pub fn build_ui(app: &adw::Application) {
    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("XDelta - LUI")
            .build(),
    );

    let notebook = Notebook::builder().show_tabs(true).build();
    let source_file = Arc::new(Mutex::new(String::new()));
    // let output_file = Rc::new(Cell::new(String::new()));
    // let target_file = Rc::new(Cell::new(String::new()));

    for i in 0..2 {
        let mode = if i == 0 { Mode::Apply } else { Mode::Create };
        let page_content = build_box();

        let label = if mode == Mode::Apply {
            Label::new(Some("Apply patch"))
        } else {
            Label::new(Some("Create patch"))
        };

        // Create buttons
        for b in 0..4 {
            let button = build_button(&mode, b);
            // let source_file_clone = source_file.clone();

            match b {
                0 => {
                    button.connect_clicked(
                        clone!(@strong source_file, @strong window => move |b| {
                            let selected_file = callbacks::choose_source(&b, Rc::clone(&window));

                            let mut source_file =
                                source_file.lock().expect("Unable to lock mutex");

                            *source_file = selected_file;
                            println!("source file: {:?}", source_file);

                        }),
                    );
                }

                1 => {
                    if mode == Mode::Apply {
                        button.connect_clicked(|_| callbacks::choose_output());
                    } else {
                        button.connect_clicked(|_| callbacks::choose_target());
                    }
                }

                2 => {
                    if mode == Mode::Apply {
                        button.connect_clicked(|_| callbacks::choose_target());
                    } else {
                        button.connect_clicked(|_| callbacks::choose_output());
                    }
                }

                3 => {
                    if mode == Mode::Apply {
                        button.connect_clicked(|_| callbacks::apply_patch());
                    } else {
                        button.connect_clicked(|_| callbacks::create_patch());
                    }
                }
                _ => (),
            };

            page_content.append(&button);
        }

        notebook.append_page(&page_content, Some(&label));
    }

    window.set_child(Some(&notebook));
    window.present();
}

fn build_button(mode: &Mode, button_index: usize) -> Button {
    let labels = if mode == &Mode::Apply {
        vec!["ROM file:", "Patch file:", "Output file:", "Apply patch"]
    } else {
        vec![
            "Original ROM:",
            "Modified ROM:",
            "Output file:",
            "Create patch",
        ]
    };

    Button::builder()
        .label(labels[button_index])
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn build_box() -> Box {
    Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
}

fn build_file_chooser<W: IsA<gtk::Window>>(
    title: &str,
    action: FileChooserAction,
    parent: Rc<W>,
) -> FileChooserDialog {
    //
    let accept_label = if action == FileChooserAction::Open {
        "Select"
    } else if action == FileChooserAction::Save {
        "Save"
    } else {
        "Open"
    };

    FileChooserDialog::new(
        Some(title),
        Some(&*parent),
        action,
        &[
            (accept_label, ResponseType::Accept),
            ("Cancel", ResponseType::Cancel),
        ],
    )
}

fn build_file_filter(patterns: &[&str]) -> FileFilter {
    let filter = FileFilter::new();

    for pat in patterns.iter() {
        filter.add_pattern(pat);
    }

    filter
}

// fn build_progress_window(message: &str) {
//     println!("{:?}", message);
// }
