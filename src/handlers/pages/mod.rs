//! The home for page handlers. A hybrid between the "model" and "view" idea in MVC.

mod about_page;
mod contact_page;
mod home_page;
mod register_page;

pub use self::about_page::about;
pub use self::contact_page::contact;
pub use self::home_page::home;
pub use self::register_page::register;
