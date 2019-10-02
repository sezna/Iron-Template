use iron::prelude::*;
use store::get_user;
use templates::components::{header, nav_bar};

use utils::html_response;

pub fn contact(r: &mut Request) -> IronResult<Response> {
    let user = Box::new(get_user(r));
    let markup = html! {
        : header(String::from("About"));
         link(rel="stylesheet", href="/files/css/about.css");
        body {
            : nav_bar(user.clone());
            div(class="main-content centered-text") {
                 h1 { : "Contact" }
                 p { : "Contact me directly by emailing ";
                        a(href="TODO") { :"TODO" }
                   }
            }
        }
        br;

    };

    Ok(html_response(markup.to_string()))
}
