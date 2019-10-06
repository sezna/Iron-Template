use router::Router;

use iron::prelude::*;

use crate::handlers;
use crate::handlers::pages::{about, contact, home, register};
use crate::handlers::template;
use crate::store::get_user;
use crate::templates;
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

    let mut router = routes!(get "/" => home,
                                      get "/about" => about,
                                      get "/contact" => contact,
                                      get "/register" => register   );

    router.post(
        "/session",
        |r: &mut Request| handlers::endpoints::session_management(r),
        "session",
    );

    router.get(
        "/login",
        |r: &mut Request| {
            handlers::template(templates::pages::generic::form::templates::log_in().render(r))
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
