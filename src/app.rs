/// The module that handles the
/// logic of the app database.
pub mod logic;
pub use logic::todo_list;

use gtk::prelude::*;
use gtk::{gio, glib, Application};

use crate::AppWindow;

/// The application id.
const APP_ID: &str = "io.github.dracoy-code.RusticTasks";

/// The main app structure.
pub struct App;

impl App {
    /// Builds and runs the app UI.
    pub fn build() -> glib::ExitCode {
        // Register the resources to build the app
        gio::resources_register_include!("resources.gresource")
            .expect("Failed to register resources.");
        
        // Initialise the app
        let application = Application::builder()
            .application_id(APP_ID)
            .build();
        
        // Connect the app to the window
        application.connect_activate(|app| {
            let window = AppWindow::new(app);
            window.present();
        });

        // Run the app
        application.run()
    }
}
