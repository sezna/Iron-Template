//! The home for page handlers. A hybrid between the "model" and "view" idea in MVC.

mod about_page;
mod contact_page;
mod home_page;
mod log_in_page;
mod register_page;
mod settings_page;

pub use self::about_page::about;
pub use self::contact_page::contact;
pub use self::home_page::home;
pub use self::log_in_page::log_in;
pub use self::register_page::register;
pub use self::settings_page::settings;
