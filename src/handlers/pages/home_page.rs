use horrorshow::helper::doctype;

use iron::prelude::*;

use store::get_user;
use templates::components::{header, nav_bar};
use utils::html_response;

pub fn home(r: &mut Request) -> IronResult<Response> {
    let user = Box::new(get_user(r));
    let markup = html!(
        :doctype::HTML;
        html {
            : header(String::from("Homepage"));
            body {
                : nav_bar(user.clone());
                div(id="title", class="centered largetext") {
                    p { :"Rust Template" }
                }
            }
        }
    );

    Ok(html_response(markup.to_string()))
}
