
use horrorshow::helper::doctype;
use horrorshow::prelude::*;

use iron::Request;
use store::{get_store, get_user, Submission};

pub fn settings(r: &mut Request) -> String {
    let markup = html! {
        : doctype::HTML;
        html {
            link(rel="stylesheet", href="/files/css/practice_sheet_form.css");
            link(rel="stylesheet", href="/files/css/about.css");
            :header(String::from("Settings"));
            body {
                : nav_bar(Box::new(user));
                br;
                br;
                br;
                br;
                div(class="main-content") {
                     form(action="/editpreferences", method="post") {
                        select(name="")
                     }
                }
            }
        }
    };
    markup.into_string().unwrap()
}