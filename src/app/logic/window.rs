/// The module that implements the AppWindow object.
mod imp;

use std::path::Path;

use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, ListItem, NoSelection, SignalListItemFactory};

use crate::{Author, LoginState, TaskBox, TaskEntry};

glib::wrapper! {
    /// Wrapper for the AppWindow GObject subclass.
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget, 
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AppWindow {
    /// Creates a new instance of AppWindow.
    pub fn new(app: &Application) -> Self {
        let window: AppWindow = Object::builder()
            .property("application", app)
            .build();

        window.imp().todo_list.set_visible(false);

        window
    }
    
    /// Retrieves the current list of tasks from the task store.
    fn get_current_tasks(&self) -> gio::ListStore {
        self.imp()
            .task_store
            .borrow()
            .clone()
            .expect("Error while loading current tasks.")
    }

    /// Creates a new task and adds it to the task list.
    fn create_new_task(&self) {
        let buffer = self.imp().task_entry.buffer();
        let content = buffer.text().to_string();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");
        
        // Create a new task entry
        let task_entry = TaskEntry::new(false, content);
        self.get_current_tasks().append(&task_entry);
    }

    /// Sets up the task list model and binds it to the task store.
    fn setup_tasks(&self) {
        // Create an empty list store
        let model = gio::ListStore::new::<TaskEntry>();

        // Replaces the store with a new one
        self.imp()
            .task_store
            .replace(Some(model));
        
        // Set the storage model
        let selection_model = NoSelection::new(
            Some(self.get_current_tasks())
        );
        self.imp()
            .task_list
            .set_model(Some(&selection_model));
    }

    /// Sets up the callback functions for UI interactions.
    fn setup_callbacks(&self) {
        self.imp()
            .task_entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.create_new_task();
            }));
        
        self.imp()
            .task_entry
            .connect_icon_release(clone!(@weak self as window => move |_, _| {
                window.create_new_task()
            }));
        
        self.imp()
            .login_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                let name = window.imp().name.text().trim().to_string();
                let email = window.imp().email.text().trim().to_string();
                let password = window.imp().password.text().trim().to_string();

                if !name.is_empty() && !email.is_empty() && !password.is_empty() {
                    let file_path = Path::new("./data/author.csv");

                    // Build the author
                    let author = Author::build(&name[..], &email[..], &password[..])
                        .expect("Login unsuccessful.");
                    
                    // Try to login
                    let login_state = Author::login(&email[..], &password[..], &file_path);
                    match login_state {
                        Ok(state @ LoginState::InvalidCredentials) => {
                            eprintln!("The user credentials are invalid.");
                            println!("login_state={:?}", state);
                            return;
                        }
                        Ok(state @ LoginState::LoggedIn) => {
                            println!("login_state={:?}", state);
                            window.imp().login_form.set_visible(false);
                            window.imp().todo_list.set_visible(true);
                        },
                        Ok(state @ LoginState::DoesNotExist) => {
                            eprintln!("The user does not exist.");
                            println!("login_state={:?}", state);
                            author.write_to_csv(file_path)
                                .expect("Error while saving login info.");
                        },
                        Err(e) => {
                            eprintln!("Error: {e}");
                        }
                    }
                    
                    window.imp().author_salutation.set_label(
                        &format!("Hello, {}!", author.get_name())
                    );
                }
            }));
    }

    /// Sets up the factory for creating and binding task items in the task list.
    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let task_box = TaskBox::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&task_box));
        });

        factory.connect_bind(move |_, list_item| {
            let task_entry = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<TaskEntry>()
                .expect("The item has to be a `TaskEntry`.");

            let task_box = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<TaskBox>()
                .expect("The child has to be a `TaskBox`.");
            
            task_box.bind(&task_entry);
        });

        factory.connect_unbind(move |_, list_item| {
            let task_box = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<TaskBox>()
                .expect("The child has to be a `TaskBox`.");

            task_box.unbind();
        });

        self.imp().task_list.set_factory(Some(&factory));
    }
}
