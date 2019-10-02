extern crate iron;
mod basic_template;
pub mod endpoints;
pub mod pages;
mod settings_page;

pub use self::basic_template::template;
pub use self::settings_page::settings;
