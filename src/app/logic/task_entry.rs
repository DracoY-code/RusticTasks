/// The module that implements the TaskEntry object.
mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    /// Wrapper for the TaskEntry GObject subclass.
    pub struct TaskEntry(ObjectSubclass<imp::TaskEntry>);
}

impl TaskEntry {
    /// Creates a new instance of TaskEntry.
    pub fn new(completed: bool, content: String) -> Self {
        Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }
}

/// The structure that maintains
/// the task entry metadata.
#[derive(Default)]
pub struct TaskData {
    pub completed: bool,
    pub content: String,
}
