mod imp;

use glib::Object;
use gtk::{gio, glib, subclass::prelude::ObjectSubclassIsExt};

use crate::lib::Files;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_files(&self) {
        self.imp().files.replace(Files {
            source: "".to_string(),
            target: "".to_string(),
            output: "".to_string(),
        });
    }

    fn setup_parent_window(&self) {
        self.imp().parent_window.replace(gtk::Window::new());
    }
}
