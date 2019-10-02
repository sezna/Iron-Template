extern crate iron;
pub mod endpoints;
mod settings_page;
mod basic_template;
pub mod pages;

pub use self::basic_template::template;
pub use self::settings_page::settings;
