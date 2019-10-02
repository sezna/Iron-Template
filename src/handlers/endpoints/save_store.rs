use iron::prelude::*;

use store::{get_store, get_user, UserRole};

use handlers::pages::home;
use utils::validate_query_params;

/// Saves the store (current state of the application) to a data file. Only works if you are logged in as an Admin.
/// Takes one parameter: `format`, which determines if we are saving to json or bincode.
pub fn save_store_endpoint(r: &mut Request) -> IronResult<Response> {
    let store = get_store(r);
    if let Some(user) = get_user(r) {
        if user.role == UserRole::Admin {
            let query_map = validate_query_params(r, vec!["format"]).unwrap();
            let format = query_map.get("format").unwrap();
            if format == "bincode" {
                println!("event: saved store via endpoint to bincode");
                store
                    .read()
                    .unwrap()
                    .save_to_storage()
                    .expect("error: critical: unable to save application state");
            } else if format == "json" {
                println!("event: saved store via endpoint to json");
                store
                    .read()
                    .unwrap()
                    .save_to_json()
                    .expect("error: critical: unable to save application state");
            } else {
                println!(
                    "event: attempted to save store with unrecognized format: {}",
                    format
                );
            }
        } else {
            println!(
                "event: rejected saved store endpoint for user {}",
                user.name
            );
        }
    }

    home(r)
}
