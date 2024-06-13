/// The module that implements the TaskBox object.
mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

use crate::TaskEntry;

glib::wrapper! {
    /// Wrapper for TaskBox GObject subclass.
    pub struct TaskBox(ObjectSubclass<imp::TaskBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for TaskBox {
    /// Creates a new default instance of TaskBox.
    fn default() -> Self {
        Self::new()
    }
}

impl TaskBox {
    /// Creates a new instance of TaskBox.
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Binds properties of a TaskEntry to this TaskBox.
    pub fn bind(&self, task_entry: &TaskEntry) {
        let completed_button = self.imp().completed_button.get();
        let content_label = self.imp().content_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind the checkbox that marks the task as completed
        let completed_button_binding = task_entry
            .bind_property(
                "completed",
                &completed_button,
                "active",
            )
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(completed_button_binding);
        
        // Bind the content label text
        let content_label_binding = task_entry
            .bind_property(
                "content",
                &content_label,
                "label"
            )
            .sync_create()
            .build();
        bindings.push(content_label_binding);

        // Bind certain attributes with the content label
        let content_label_binding = task_entry
            .bind_property(
                "completed",
                &content_label,
                "attributes"
            )
            .sync_create()
            .transform_to(|_, active| {
                let attribute_list = AttrList::new();
                if active {
                    let attribute = AttrInt::new_strikethrough(true);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            })
            .build();
        bindings.push(content_label_binding);
    }

    /// Unbinds all previously established property bindings.
    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
