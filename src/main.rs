#[macro_use]
extern crate horrorshow;
extern crate bincode;
extern crate chrono;
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate ring;
extern crate router;
extern crate secure_session;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate snowflake;
extern crate staticfile;
extern crate url;
extern crate urlencoded;
use iron::prelude::*;

use mount::Mount;
use secure_session::middleware::{SessionConfig, SessionMiddleware};
use secure_session::session::ChaCha20Poly1305SessionManager;
use staticfile::Static;

use crate::store::{configure_store, Session, SessionKey};

mod handlers;
mod routes;
mod store;
mod templates;
mod utils;

fn main() {
    let project_directory = env!("CARGO_MANIFEST_DIR");
    let router = routes::build_router();
    let files_path = format!("{}{}", project_directory, "/resources/files");

    let store = configure_store().expect("Error loading store.");
    // Middleware and static file mounting
    let mut chain = Chain::new(router);
    chain.link_before(store);

    // session management
    let key = *b"84730567777778471123456700938567";
    let manager = ChaCha20Poly1305SessionManager::<Session>::from_key(key);
    let config = SessionConfig::default();
    let session_middleware =
        SessionMiddleware::<Session, SessionKey, ChaCha20Poly1305SessionManager<Session>>::new(
            manager, config,
        );

    chain.link_around(session_middleware);
    let mut mount = Mount::new();
    mount
        .mount("/", chain)
        .mount("/files/", Static::new(files_path));
    println!("Server running at port 80");
    Iron::new(mount)
        .http("localhost:3000")
        .expect("unable to mount server");
}
