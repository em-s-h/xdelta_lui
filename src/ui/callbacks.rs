use glib::clone;
use gtk::{glib, prelude::*, Button, FileChooserAction, ResponseType};
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

pub fn choose_source<W: IsA<gtk::Window>>(button: &Button, parent: Rc<W>) -> String {
    let file_chooser = super::build_file_chooser("Choose source", FileChooserAction::Open, parent);
    let filter = super::build_file_filter(&["*.png"]);

    let selected_file = Arc::new(Mutex::new(String::new()));

    file_chooser.connect_response(clone!(@strong selected_file => move |d, r| {
        if r == ResponseType::Accept {
            if let Some(file) = d.file() {
                let mut selected_file =
                    selected_file.lock().expect("Unable to lock mutex");

                *selected_file = file.to_string();
                // button.set_label(&selected_file);
                println!("{selected_file}");
            }
        }
        d.destroy();
    }));

    file_chooser.set_filter(&filter);
    file_chooser.show();

    let selected_file = selected_file
        .lock()
        .expect("Unable to lock mutex")
        .to_string();

    selected_file
}

pub fn choose_target() {}

pub fn choose_output() {}

pub fn create_patch() {}

pub fn apply_patch() {}
