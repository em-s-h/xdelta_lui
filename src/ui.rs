use glib::clone;
use gtk::{
    glib, prelude::*, ApplicationWindow, Box, Button, FileChooserAction, FileChooserDialog,
    FileFilter, Label, Notebook, Orientation, ResponseType,
};
use std::{cell::Cell, rc::Rc};

mod callbacks;

const ROM_PREFIXES: &[&str] = &["*.nes", "*.gb", "*.sfc", "*.gba", "*.nds", "*.iso"];
const PATCH_PREFIX: &[&str] = &["*.xdelta"];

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Create,
    Apply,
}

pub fn build_ui(app: &adw::Application) {
    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("XDelta - LUI")
            .build(),
    );

    let notebook = Notebook::builder().show_tabs(true).build();
    let source_file = Rc::new(Cell::new(String::new()));
    let output_file = Rc::new(Cell::new(String::new()));
    let target_file = Rc::new(Cell::new(String::new()));

    for i in 0..2 {
        let mode;
        let label;

        if i == 0 {
            label = Label::new(Some("Apply patch"));
            mode = Mode::Apply;
        } else {
            label = Label::new(Some("Create patch"));
            mode = Mode::Create;
        };
        let page_content = build_box();

        // Create buttons
        for b in 0..4 {
            let button = build_button(&mode, b);

            match b {
                0 => {
                    button.connect_clicked(
                        clone!(@strong source_file, @strong window => move |b| {
                            callbacks::open_file_chooser(
                                &b,
                                Rc::clone(&window),
                                Rc::clone(&source_file),
                                FileChooserAction::Open,
                                ROM_PREFIXES,
                            );
                        }),
                    );
                }

                1 => {
                    // target: xdelta file
                    if mode == Mode::Apply {
                        button.connect_clicked(
                            clone!(@strong target_file, @strong window => move |b| {
                                callbacks::open_file_chooser(
                                    &b,
                                    Rc::clone(&window),
                                    Rc::clone(&target_file),
                                    FileChooserAction::Open,
                                    PATCH_PREFIX
                                );
                            }),
                        );
                    // output: modified rom
                    } else {
                        button.connect_clicked(
                            clone!(@strong output_file, @strong window => move |b| {
                                callbacks::open_file_chooser(
                                    &b,
                                    Rc::clone(&window),
                                    Rc::clone(&output_file),
                                    FileChooserAction::Open,
                                    ROM_PREFIXES
                                );
                            }),
                        );
                    }
                }

                2 => {
                    // output: modified rom
                    if mode == Mode::Apply {
                        button.connect_clicked(
                            clone!(@strong output_file, @strong window => move |b| {
                                callbacks::open_file_chooser(
                                    &b,
                                    Rc::clone(&window),
                                    Rc::clone(&output_file),
                                    FileChooserAction::Save,
                                    ROM_PREFIXES
                                );
                            }),
                        );
                    // target xdelta file
                    } else {
                        button.connect_clicked(
                            clone!(@strong target_file, @strong window => move |b| {
                                callbacks::open_file_chooser(
                                    &b,
                                    Rc::clone(&window),
                                    Rc::clone(&target_file),
                                    FileChooserAction::Save,
                                    PATCH_PREFIX
                                );
                            }),
                        );
                    }
                }

                3 => {
                    if mode == Mode::Apply {
                        button.connect_clicked(
                            clone!(@strong source_file, @strong target_file, @strong output_file =>
                                move |_| callbacks::call_xdelta(
                                    Rc::clone(&source_file),
                                    Rc::clone(&target_file),
                                    Rc::clone(&output_file),
                                    Mode::Apply
                                );
                            ),
                        );
                    } else {
                        button.connect_clicked(
                            clone!(@strong source_file, @strong target_file, @strong output_file =>
                                move |_| callbacks::call_xdelta(
                                    Rc::clone(&source_file),
                                    Rc::clone(&target_file),
                                    Rc::clone(&output_file),
                                    Mode::Create
                                );
                            ),
                        );
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
    parent: Rc<W>,
    action: FileChooserAction,
) -> FileChooserDialog {
    //
    FileChooserDialog::new(
        Some(title),
        Some(&*parent),
        action,
        &[
            ("Select", ResponseType::Accept),
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
