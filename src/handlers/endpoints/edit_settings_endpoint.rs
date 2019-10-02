use handlers::settings;
use iron::prelude::*;
use iron::status;
use store::preferences::Preferences;
use store::{get_store, get_user};
use templates;
use utils::{get_body_parameters, html_response};
use handlers::pages::home;

pub fn edit_settings(r: &mut Request) -> IronResult<Response> {
    let params = get_body_parameters(r);

    let user_id = if let Some(user) = get_user(r) {
        user.id
    } else {
       return home(r);
    };

    let preferences = Preferences {
    };

    get_store(r)
        .write()
        .unwrap()
        .set_user_preferences(user_id, preferences); //users.iter().find(|x: &&User| x.id == user_id).expect("no user found for id");

    // The code that calls this endpoint doesn't redirect, so it doesn't matter where we go. That being said, let's just bounce back to the settings page just in case.
    return settings(r);
}
