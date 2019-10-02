/// The home for api-style endpoints. Here is where you'll typically implement form action targets and the like
/// which manage the application store.

pub use self::edit_settings_endpoint::edit_settings;
pub use self::logout::logout_endpoint;

pub use self::save_store::save_store_endpoint;
pub use self::session::session_management;

mod edit_settings_endpoint;
mod logout;
mod save_store;
mod session;
