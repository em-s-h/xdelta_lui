mod window;
mod lib;

use std::process::{ Command, self };
use gtk::{gio, glib, prelude::*};
use window::Window;

const APP_ID: &str = "com.github.em-s-h.xdelta-lui";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("xdelta-lui.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &adw::Application) {
    verify_xdelta();
    let window = Window::new(app);
    window.present();
}

/// Verify if xdelta3 is installed.
fn verify_xdelta() {
    let status = Command::new("xdelta3").arg("-V").status()
        .expect("Unable to execute process 'xdelta3 -V");

    if status.code().unwrap_or(0) == 127 {
        eprintln!("xdelta3 is not installed! Please install it!");
        process::exit(1);
    }
}
