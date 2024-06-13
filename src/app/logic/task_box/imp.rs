use std::cell::RefCell;

use glib::{subclass::InitializingObject, Binding};
use gtk::subclass::prelude::*;
use gtk::{glib, CheckButton, CompositeTemplate, Label};

/// The TaskBox widget represents a graphical component
/// for displaying and interacting with a task.
#[derive(CompositeTemplate, Default)]
#[template(resource = "/io/github/dracoy-code/RusticTasks/task_box.ui")]
pub struct TaskBox {
    #[template_child]
    pub completed_button: TemplateChild<CheckButton>,
    #[template_child]
    pub content_label: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TaskBox {
    const NAME: &'static str = "TaskBox";
    type Type = super::TaskBox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TaskBox {}

impl WidgetImpl for TaskBox {}

impl BoxImpl for TaskBox {}
