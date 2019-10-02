mod about_page;
mod contact_page;
/// The home for page handlers. A hybrid between the "model" and "view" idea in MVC.
mod home_page;

pub use self::about_page::about;
pub use self::contact_page::contact;
pub use self::home_page::home;
