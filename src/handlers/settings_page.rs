use iron::prelude::*;

use handlers::pages;
use store::get_user;
use templates::pages::generic::form::{FieldType, Form, FormField};
use utils::html_response;

pub fn settings(r: &mut Request) -> IronResult<Response> {
    let resp = if let Some(user) = get_user(r) {
        let _current_preferences = user.preferences.clone();
        html_response(
            Form::new("Settings", vec![], "/editsettings", "Save").render(Box::new(Some(user))),
        )
    } else {
        pages::home(r).unwrap()
    };
    Ok(resp)
}
