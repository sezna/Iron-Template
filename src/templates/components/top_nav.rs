use horrorshow::prelude::*;

use store::{User, UserRole};

pub fn nav_bar(user: Box<Option<User>>) -> Box<dyn RenderBox> {
    box_html! {
         link(rel="stylesheet", href="/files/css/nav_bar.css");
         div(class="top-nav") {
            span(class="nav-item") {
                a (href="/") { : "Home" }
            }
            span(class="nav-item") {
                a (href="/about") { : "About" }
            }
            div(class="dropdown-container", onclick="void(0)") {
                span(class="nav-item", onclick="void(0)") {
                    : "Tools"
                }
                div(class="dropdown-items") {
                    a(href="/buildpracticesheet") { : "Handwriting Practice Sheet"}
                    a(href="/aboutdefinitionsubmissions") { : "Submit a Definition"}
                }
            }
            span(class="nav-item") {
                a(href="contact") { : "Contact" }
            }
            // Admin section
            @ if user.is_some() && user.clone().unwrap().role == UserRole::Admin {
                div(class="dropdown-container", onclick="void(0)") {
                    span(class="nav-item", onclick="void(0)") {
                        : "Admin"
                    }
                    div(class="dropdown-items") {
                        a(href="/savestore?format=bincode") { : "Save Store" }
                        a(href="/savestore?format=json") { : "Save Store to Json" }

                    }
                }
            } else if user.is_none() {
                span(class="nav-item") {
                    a(href="/login") { : "Login" }
                }
                span(class="nav-item") {
                    a(href="/register") { : "Register" }
                }
            }
            // Logged in user options
            @ if user.is_some() {
                div(class="dropdown-container", onclick="void(0)") {
                    span(class="nav-item") {
                        : user.clone().unwrap().name
                    }
                    div(class="dropdown-items", onclick="void(0)") {
                        a(href="/logout") { : "Log out" }
                        a(href="/settings") { : "Settings" } // TODO

                    }
                }
            }

         }
    }
}
