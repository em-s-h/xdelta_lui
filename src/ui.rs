use crate::lib::{Files, Mode};
use gtk::{
    prelude::*, ApplicationWindow, Box, Button, FileChooserAction, FileChooserNative, FileFilter,
    Label, Notebook, Orientation,
};
use std::{cell::Cell, rc::Rc};

mod callbacks;

pub fn build_ui(app: &adw::Application) {
    let files = Rc::new(Cell::new(Files::new()));

    let notebook = create_notebook();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("XDelta - LUI")
        .child(&notebook)
        .build();

    window.present();
}

fn create_notebook() -> Notebook {
    let apply_patch_page = create_page(Mode::Apply);
    let create_patch_page = create_page(Mode::Create);

    let notebook = Notebook::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .show_tabs(true)
        .show_border(true)
        .build();

    notebook.append_page(&apply_patch_page.0, Some(&apply_patch_page.1));
    notebook.append_page(&create_patch_page.0, Some(&create_patch_page.1));

    notebook
}

fn create_page(mode: Mode) -> (Box, Label) {
    let page_box = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let label;

    if mode == Mode::Apply {
        create_buttons(mode, &page_box);
        label = Label::new(Some("Apply patch"));
    } else {
        create_buttons(mode, &page_box);
        label = Label::new(Some("Create patch"));
    }

    (page_box, label)
}

fn create_buttons(mode: Mode, container: &Box) {
    let labels = if mode == Mode::Apply {
        vec!["ROM file:", "Patch file:", "Output file:", "Apply patch"]
    } else {
        vec![
            "Original ROM:",
            "Modified ROM:",
            "Output file:",
            "Create patch",
        ]
    };

    for i in 0..4 {
        let button = Button::builder()
            .label(labels[i])
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        match i {
            0 => {
                button.connect_clicked(|_| callbacks::choose_source());
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

        container.append(&button);
    }
}

fn build_file_chooser(
    title: &str,
    action: FileChooserAction,
    parent: &impl IsA<gtk::Window>,
    filter: &FileFilter,
) -> FileChooserNative {
    //
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
