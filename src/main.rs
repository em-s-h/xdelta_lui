use gdk::Display;
use gtk::{gdk, glib, prelude::*, CssProvider};
// use std::process::Command;

/// Module for building ui elements
mod ui;

const APP_ID: &str = "com.github.em-s-h.xdelta-lui";

fn main() -> glib::ExitCode {
    // verify_xdelta();

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(ui::build_ui);
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

// fn verify_xdelta() {
//     Command::new("xdelta3")
//         .arg("-q")
//         .spawn()
//         .expect("Xdelta3 is not installed! Please install it!");
// }
