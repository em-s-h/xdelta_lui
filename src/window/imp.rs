use gtk::{glib, CompositeTemplate, subclass::prelude::*, Button, prelude::StaticTypeExt, Notebook, NotebookTab, Label};
use glib::subclass::InitializingObject;
use crate::lib::Files;
use std::cell::Cell;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/em-s-h/xdelta-lui/window.ui")]
pub struct Window {
    pub files: Cell<Files>,
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn choose_source(&self, button: &Button) {
        println!("{:?}", button);
    }

    #[template_callback]
    fn choose_target(&self, button: &Button) {
        println!("{:?}", button);
    }

    #[template_callback]
    fn choose_output(&self, button: &Button) {
        println!("{:?}", button);
    }

    #[template_callback]
    fn create_patch(&self, button: &Button) {
        println!("{:?}", button);
    }

    #[template_callback]
    fn apply_patch(&self, button: &Button) {
        println!("{:?}", button);
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match 'class' attribute of template
    const NAME: &'static str = "XdeltaWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

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

#[cfg(test)]
mod test {
}
