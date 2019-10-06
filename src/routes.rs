use router::Router;

use iron::prelude::*;

use crate::handlers;
use crate::handlers::endpoints::{edit_settings, log_out, save_store, session_management};
use crate::handlers::pages::{about, contact, home, log_in, register, settings};

/*
routes!( get "/" => homepage, get "/about" => about, get "/contact" => contact );

... expands into ...

let mut router = Router::new();
router.get("/", |r: &mut Request| homepage(r), "/");
router.get("/about", |r: &mut Request| about(r), "/about");
router.get("/contact", |r: &mut Request| contact(r), "/contact");

*/

macro_rules! routes {
    (
        $(
            $method:ident $path:expr => $destination:expr
         ),+
    ) => {
            {
                let mut router = Router::new();
                $(
                    match stringify!($method).to_string().as_str()  {
                        "get" =>  { router.get($path,  |r: &mut Request| $destination(r), $path); },
                        "post" => { router.post($path, |r: &mut Request| $destination(r), $path); },
                        _ => {}
                    };
                )+
                router
            }
    }
}

pub fn build_router() -> Router {
    // Routes
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
