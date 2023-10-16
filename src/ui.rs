use glib::clone;
use gtk::{
    glib, prelude::*, ApplicationWindow, Box, Button, FileChooserAction, FileFilter, Label,
    Notebook, Orientation,
};
use std::{cell::Cell, rc::Rc};

/// Module for handling callbacks
mod callbacks;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Create,
    Apply,
}

const ROM_PREFIXES: &[&str] = &["*.nes", "*.gb", "*.sfc", "*.gba", "*.nds", "*.3ds", "*.iso"];
const PATCH_PREFIX: &[&str] = &["*.xdelta"];

pub fn build_ui(app: &adw::Application) {
    // {{{
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

    // Create tabs {{{
    for i in 0..2 {
        let operation;
        let label;

        if i == 0 {
            label = Label::new(Some("Apply patch"));
            operation = Operation::Apply;
        } else {
            label = Label::new(Some("Create patch"));
            operation = Operation::Create;
        };
        let page_content = build_box();

        // Create buttons {{{
        for b in 0..4 {
            let button = build_button(&operation, b);

            match b {
                // source {{{
                0 => {
                    button.connect_clicked(
                        clone!(@strong source_file, @strong window => move |b| {
                            callbacks::open_file_chooser(
                                Rc::clone(&window),
                                &b,
                                Rc::clone(&source_file),
                                FileChooserAction::Open,
                                ROM_PREFIXES,
                            );
                        }),
                    );
                }
                // }}}
                // target | output {{{
                1 => {
                    // target: xdelta file
                    if operation == Operation::Apply {
                        button.connect_clicked(
                            clone!(@strong target_file, @strong window => move |b| {
                                callbacks::open_file_chooser(
                                    Rc::clone(&window),
                                    &b,
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
                                    Rc::clone(&window),
                                    &b,
                                    Rc::clone(&output_file),
                                    FileChooserAction::Open,
                                    ROM_PREFIXES
                                );
                            }),
                        );
                    }
                }
                // }}}
                // output | target {{{
                2 => {
                    // output: modified rom
                    if operation == Operation::Apply {
                        button.connect_clicked(
                            clone!(@strong output_file, @strong window => move |b| {
                                callbacks::open_file_chooser(
                                    Rc::clone(&window),
                                    &b,
                                    Rc::clone(&output_file),
                                    FileChooserAction::Save,
                                    ROM_PREFIXES
                                );
                            }),
                        );
                    // target: xdelta file
                    } else {
                        button.connect_clicked(
                            clone!(@strong target_file, @strong window => move |b| {
                                callbacks::open_file_chooser(
                                    Rc::clone(&window),
                                    &b,
                                    Rc::clone(&target_file),
                                    FileChooserAction::Save,
                                    PATCH_PREFIX
                                );
                            }),
                        );
                    }
                }
                // }}}
                // apply | create {{{
                3 => {
                    button.add_css_class("suggested-action");
                    button.connect_clicked(clone!(
                        @strong window,
                        @strong source_file,
                        @strong target_file,
                        @strong output_file,
                        @strong operation
                            => move |_| callbacks::call_xdelta(
                                Rc::clone(&window),
                                Rc::clone(&source_file),
                                Rc::clone(&target_file),
                                Rc::clone(&output_file),
                                &operation
                            );
                    ));
                }
                // }}}
                _ => (),
            };

            page_content.append(&button);
        }
        // }}}

        notebook.append_page(&page_content, Some(&label));
    }
    // }}}

    window.set_child(Some(&notebook));
    window.present();
    // }}}
}

fn build_button(operation: &Operation, button_index: usize) -> Button {
    // {{{
    let labels = if operation == &Operation::Apply {
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
    // }}}
}

fn build_box() -> Box {
    // {{{
    Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build()
    // }}}
}

pub fn build_file_filter(patterns: &[&str]) -> FileFilter {
    // {{{
    let filter = FileFilter::new();

    for pat in patterns.iter() {
        filter.add_pattern(pat);
    }

    filter
    // }}}
}
