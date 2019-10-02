use router::Router;
use std::sync::Arc;

use iron::prelude::*;

use handlers::template;
use store::get_user;
use handlers::pages::{about, contact, home};
use templates;
use handlers;
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
pub fn build_configured_router() -> Router {
    // Routes

    let mut router = routes!( get "/" => home,
                                      get "/about" => about,
                                      get "/contact" => contact );

    router.get(
        "/register",
        |r: &mut Request| {
            template(
                templates::pages::generic::form::templates::register()
                    .render(Box::new(get_user(r))),
            )
        },
        "registerform",
    );

    router.post(
        "/session",
        |r: &mut Request| handlers::endpoints::session_management(r),
        "session",
    );

    router.get(
        "/login",
        |r: &mut Request| {
            handlers::template(
                templates::pages::generic::form::templates::log_in().render(Box::new(get_user(r))),
            )
        },
        "login",
    );

    router.get(
        "/savestore",
        |r: &mut Request| handlers::endpoints::save_store_endpoint(r),
        "savestore",
    );

    router.get(
        "/logout",
        |r: &mut Request| handlers::endpoints::logout_endpoint(r),
        "logout",
    );

    router.get(
        "/settings",
        |r: &mut Request| handlers::settings(r),
        "settings",
    );

    router.post(
        "/editsettings",
        |r: &mut Request| handlers::endpoints::edit_settings(r),
        "editsettings",
    );

    return router;
}
