use crate::store::{get_store, SessionKey};
use iron::{IronResult, Request, Response};

use crate::handlers::pages;

/// To log out, you only need the username, but your session ID is authenticated against
/// the current sessions, so you cannot log out other users.

pub fn log_out(r: &mut Request) -> IronResult<Response> {
    let session_id = r.extensions.remove::<SessionKey>();
    match session_id {
        Some(session) => {
            get_store(r).write().unwrap().sessions.remove(&session.id);
        }
        None => (),
    }
    pages::home(r)
}
