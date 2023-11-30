use glib::clone;
use gtk::{
    glib, prelude::*, ApplicationWindow, Box, Button, FileChooserAction, Label, Notebook,
    Orientation,
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
const PATCH_PREFIX: &[&str] = &["*.xdelta", "*.xdelta3"];

const APPLY_BUTTONS_LABELS: &[&str] = &["ROM file:", "Patch file:", "Output file:", "Apply patch"];
const CREATE_BUTTONS_LABELS: &[&str] = &[
    "Original ROM:",
    "Modified ROM:",
    "Output file:",
    "Create patch",
];

fn create_buttons(operation: Operation) -> Vec<Button> {
    // {{{
    let mut buttons: Vec<Button> = Vec::new();
    for b in 0..4 {
        let labels = if operation == Operation::Apply {
            APPLY_BUTTONS_LABELS
        } else {
            CREATE_BUTTONS_LABELS
        };

        let button = Button::builder()
            .label(labels[b])
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        buttons.push(button);
    }
    buttons
}
// }}}

pub fn build_ui(app: &adw::Application) {
    // {{{
    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("XDelta - LUI")
            .build(),
    );

    let apply_buttons_refv = create_buttons(Operation::Apply);
    let create_buttons_refv = create_buttons(Operation::Create);

    let notebook = Notebook::builder().show_tabs(true).build();
    let source_file = Rc::new(Cell::new(String::new()));
    let output_file = Rc::new(Cell::new(String::new()));
    let target_file = Rc::new(Cell::new(String::new()));

    // Create tabs {{{
    for i in 0..2 {
        let operation;
        let buttons;
        let label;

        if i == 0 {
            label = Label::new(Some("Apply patch"));
            operation = Operation::Apply;
            buttons = &apply_buttons_refv;
        } else {
            label = Label::new(Some("Create patch"));
            operation = Operation::Create;
            buttons = &create_buttons_refv;
        };
        let page_content = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        // Done to avoid using buttons in the for loop, since it will be used inside of it
        let buttons_iter = buttons.iter();

        // Add buttons to the page {{{
        for (id, button) in buttons_iter.enumerate() {
            match id {
                // source: original rom {{{
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
                    @strong buttons,
                    @strong operation
                    => move |_| {
                        callbacks::call_xdelta(
                            Rc::clone(&window),
                            Rc::clone(&source_file),
                            Rc::clone(&target_file),
                            Rc::clone(&output_file),
                            operation.clone()
                        );

                        // Reset the labels of the buttons
                        for (id,button) in buttons.iter().enumerate() {
                            let labels = if operation == Operation::Apply {
                                APPLY_BUTTONS_LABELS
                            } else {
                                CREATE_BUTTONS_LABELS
                            };
                            button.set_label(labels[id])
                        }
                    }
                    ));
                }
                // }}}
                _ => (),
            };

            page_content.append(button);
        }
        // }}}

        notebook.append_page(&page_content, Some(&label));
    }
    // }}}

    window.set_child(Some(&notebook));
    window.present();
}
// }}}
