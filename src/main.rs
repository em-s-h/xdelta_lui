use gtk::{gio::Cancellable, glib, prelude::*, AlertDialog, ApplicationWindow};
use std::{
    process::{self, Command},
    rc::Rc,
};

/// Module for building ui elements
mod ui;

const APP_ID: &str = "com.github.em-s-h.xdelta-lui";

fn main() -> glib::ExitCode {
    // {{{
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.connect_startup(verify_xdelta);
    app.run()
    // }}}
}

fn verify_xdelta(app: &adw::Application) {
    // {{{
    let status = Command::new("xdelta3").arg("-V").status();

    if let Ok(s) = status {
        if s.success() {
            return;
        }
    }

    let details = format!(
        "xdelta3 was not found! Please make sure it is installed. \n{}",
        status.unwrap_err().to_string()
    );
    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("XDelta - LUI")
            .build(),
    );

    let dialog = AlertDialog::builder()
        .detail(details)
        .message("Error!")
        .modal(true)
        .build();

    let parent = Rc::clone(&window);
    dialog.choose(Some(&*parent), Some(&Cancellable::new()), |_| {
        process::exit(1)
    });
    // }}}
}
