use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::TaskData;

/// This structure maintains the data in the TaskBox.
#[derive(Default, Properties)]
#[properties(wrapper_type = super::TaskEntry)]
pub struct TaskEntry {
    #[property(name = "completed", get, set, type = bool, member = completed)]
    #[property(name = "content", get, set, type = String, member = content)]
    pub metadata: RefCell<TaskData>,
}

#[glib::object_subclass]
impl ObjectSubclass for TaskEntry {
    const NAME: &'static str = "TaskEntry";
    type Type = super::TaskEntry;
}

#[glib::derived_properties]
impl ObjectImpl for TaskEntry {}
