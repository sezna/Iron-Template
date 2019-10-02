use store::User;
use templates::components::{header, nav_bar};

// TODO better welcome page?
pub fn welcome(user: Box<Option<User>>) -> String {
    let markup = html! {
        : header(String::from("Welcome!"));
         link(rel="stylesheet", href="/files/css/about.css");
        body {
            : nav_bar(user.clone());
            div(class="main-content") {
               h1 { :"Welcome!" }
               p { : "Feel free to look around. You can read more about this site ";
                    a(href="/about") { : "on our about page. "}
                }
            }
        }

        br;

    };

    return markup.to_string();
}
