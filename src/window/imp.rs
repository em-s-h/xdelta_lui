use gtk::{glib, CompositeTemplate, subclass::prelude::*};
use glib::subclass::InitializingObject;

const buffer_size: u32 = 1000000000;
const apply_patch_args: &str = "";
const create_patch_args: &str = "";

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/em-s-h/xdelta-lui/window.ui")]
pub struct Window {
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
