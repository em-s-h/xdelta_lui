use gtk::{glib, prelude::*, ApplicationWindow, MessageType};
use std::{
    process::{self, Command},
    rc::Rc,
};

/// Module for building ui elements
mod ui;

const APP_ID: &str = "com.github.em-s-h.xdelta-lui";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.connect_startup(verify_xdelta);
    app.run()
}

fn verify_xdelta(app: &adw::Application) {
    let status = Command::new("xdelta3").arg("-q").status();

    if let Ok(s) = status {
        if s.success() {
            return;
        }
    }

    let message = format!(
        "xdelta3 was not found! Please make sure it is installed. \n{}",
        status.unwrap_err().to_string()
    );
    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("XDelta - LUI")
            .build(),
    );

    let dialog = ui::build_dialog(Rc::clone(&window), MessageType::Error, &message);
    dialog.connect_response(|obj, _| {
        obj.destroy();
        process::exit(1);
    });
    dialog.show();
}
