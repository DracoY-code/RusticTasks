/// The module that handles the functionalities
/// of the app GUI and the app logic.
pub mod app;
pub use app::App;
pub use app::logic::{
    AppWindow,
    Author,
    LoginState,
    Priority,
    Status,
    Task,
    TaskBox,
    TaskEntry,
    ToDoList,
};

/// The module that handles the
/// error handling in the app.
pub mod error;
pub use error::{
    Argon2Error,
    RusticError,
};
