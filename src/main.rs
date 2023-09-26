use gtk::{glib, prelude::*};
use std::process::{self, Command};

mod ui;

const APP_ID: &str = "com.github.em-s-h.xdelta-lui";

fn main() -> glib::ExitCode {
    verify_xdelta();

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run()
}

/// Verify if xdelta3 is installed.
fn verify_xdelta() {
    let status = Command::new("xdelta3")
        .arg("-V")
        .status()
        .expect("Unable to execute process 'xdelta3 -V");

    if status.code().unwrap_or(0) == 127 {
        eprintln!("xdelta3 is not installed! Please install it!");
        process::exit(1);
    }
}
