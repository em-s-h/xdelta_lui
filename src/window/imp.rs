use gtk::{glib, CompositeTemplate, subclass::prelude::*, Button};
use glib::subclass::InitializingObject;

const BUFFER_SIZE: u32 = 1000000000;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/em-s-h/xdelta-lui/window.ui")]
pub struct Window {
    pub sorce_file: String,
    pub target_file: String,
    pub output_file: String,
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn choose_source(&self, button: &Button) {
        unimplemented!();
    }

    #[template_callback]
    fn choose_target(&self, button: &Button) {
        unimplemented!();
    }

    #[template_callback]
    fn choose_output(&self, button: &Button) {
        unimplemented!();
    }

    #[template_callback]
    fn create_patch(&self, button: &Button) {
        unimplemented!();
    }

    #[template_callback]
    fn apply_patch(&self, button: &Button) {
        unimplemented!();
    }

    fn build_file_chooser() {
        unimplemented!();
    }

    fn build_progress_window(message: &str) {
        unimplemented!();
    }

    fn get_file_name(path: &str) -> &str {
        unimplemented!();
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "Window";
    type ParentType = gtk::ApplicationWindow;
    type Type = super::Window;

    fn class_init(class: &mut Self::Class) {
        class.bind_template();
        class.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
