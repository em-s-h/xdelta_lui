use gtk::{glib, prelude::*};
use std::process::Command;

/// Module for building ui elements
mod ui;

const APP_ID: &str = "com.github.em-s-h.xdelta-lui";

fn main() -> glib::ExitCode {
    verify_xdelta();

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run()
}

fn verify_xdelta() {
    Command::new("xdelta3")
        .arg("-q")
        .spawn()
        .expect("Xdelta3 is not installed! Please install it!");
}
