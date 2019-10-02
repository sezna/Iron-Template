/// This module is the home for any generic pages. These typically take the form of a struct with a "render" method.
/// The struct contains the data the generic page needs, and the render method turns it all into an actual page.
pub mod form;
pub use self::form::Form;
