use crate::store::get_user;
use crate::templates::components::{header, nav_bar};

use iron::prelude::*;
use crate::utils::html_response;
pub fn about(r: &mut Request) -> IronResult<Response> {
    let user = Box::new(get_user(r));
    let markup = html! {
        : header(String::from("About"));
         link(rel="stylesheet", href="/files/css/about.css");
        body {
            : nav_bar(user.clone());
            div(class="main-content") {
               h1 { :"About" }
                 p { : "This is an example template using Iron with a user system, persistent store, and and session management." }
            }
        }

        br;

    };

    Ok(html_response(markup.to_string()))
}
