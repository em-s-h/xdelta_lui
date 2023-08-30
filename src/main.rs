mod window;

use gtk::{gio, prelude::*};
use window::Window;

const APP_ID: &str = "com.github.em-s-h.xdelta-lui";

fn main() {
    // Register and include resources
    gio::resources_register_include!("xdelta-lui.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);
    window.present();
}
