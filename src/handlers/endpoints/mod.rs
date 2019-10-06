/// The home for api-style endpoints. Here is where you'll typically implement form action targets and the like
/// which manage the application store.
pub use self::edit_settings_endpoint::edit_settings;
pub use self::log_out_endpoint::log_out;

pub use self::save_store_endpoint::save_store;
pub use self::session::session_management;

mod edit_settings_endpoint;
mod log_out_endpoint;
mod save_store_endpoint;
mod session;
