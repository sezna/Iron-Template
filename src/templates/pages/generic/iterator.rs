// Displays a generic iterator of renderable items.

use horrorshow::helper::doctype;
use horrorshow::prelude::*;

pub fn display_iterable(content: Vec<dyn Render>) -> String {
    let markup = html! {
        @ for item in content {
            : item
        }
    };
    markup.into_string().unwrap()
}