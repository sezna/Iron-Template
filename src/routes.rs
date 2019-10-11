use router::Router;

use iron::prelude::*;

use ::handlers;
use ::handlers::endpoints::{edit_settings, log_out, save_store, session_management};
use ::handlers::pages::{about, contact, home, log_in, register, settings};
use ::routes;

pub fn build_router() -> Router {
    #[rustfmt::skip]
    let mut router = routes!( get  "/" => home,
                              get  "/about" => about,
                              get  "/contact" => contact,
                              get  "/register" => register,
                              post "/session" => session_management,
                              get  "/log-in" => log_in,
                              get  "/save-store" => save_store,
                              get  "/log-out" => log_out,
                              get  "/settings" => settings,
                              post "/edit-settings" => edit_settings);
    return router;
}
