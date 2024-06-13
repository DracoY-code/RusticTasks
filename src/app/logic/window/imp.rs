use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ApplicationWindow, Button, CompositeTemplate, Entry, Label, ListView};

/// The structure that represents the main application window.
#[derive(CompositeTemplate, Default)]
#[template(resource = "/io/github/dracoy-code/RusticTasks/app_window.ui")]
pub struct AppWindow {
    #[template_child]
    pub main_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub login_form: TemplateChild<gtk::Box>,
    #[template_child]
    pub login_salutation: TemplateChild<Label>,
    #[template_child]
    pub name: TemplateChild<Entry>,
    #[template_child]
    pub email: TemplateChild<Entry>,
    #[template_child]
    pub password: TemplateChild<Entry>,
    #[template_child]
    pub login_button: TemplateChild<Button>,
    #[template_child]
    pub todo_list: TemplateChild<gtk::Box>,
    #[template_child]
    pub author_salutation: TemplateChild<Label>,
    #[template_child]
    pub task_entry: TemplateChild<Entry>,
    #[template_child]
    pub task_list: TemplateChild<ListView>,
    pub task_store: RefCell<Option<gio::ListStore>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppWindow {
    const NAME: &'static str = "AppWindow";
    type Type = super::AppWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AppWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_tasks();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

impl WidgetImpl for AppWindow {}

impl WindowImpl for AppWindow {}

impl ApplicationWindowImpl for AppWindow {}
