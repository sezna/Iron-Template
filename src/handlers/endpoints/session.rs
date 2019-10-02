use iron::{IronResult, Request, Response};
use std::process::Command;
use crate::store::preferences::Preferences;
use crate::store::user::UserRole;
use crate::store::{get_store, get_user, Session, SessionKey};

use crate::handlers;
use crate::handlers::template;
use snowflake::ProcessUniqueId;
use crate::templates::pages;
use crate::templates::pages::generic::form;
use crate::utils::get_body_parameters;
use crate::utils::security::{hash_match, hash_password};
/// The endpoint to log in or register. Requires three things in the body: The username,
/// the password (if logging in), and the action "register" or "login". To register you also need
/// an email and password confirmation (pconfirm).

pub fn session_management(r: &mut Request) -> IronResult<Response> {
    let params = get_body_parameters(r);
    let action = params.get("action").expect("no action param");

    let store = get_store(r);
    let _ = store.read().unwrap().save_to_storage();

    match action.as_str() {
        "login" => {
            let username = params.get("username").expect("no username in body");
            let password = params.get("password").expect("no password in body");
            // This _should_ be an option type but I couldn't figure out how to turn an Option<&User> into an Option<User>.
            let option_user = {
                get_store(r)
                    .read()
                    .unwrap()
                    .users
                    .iter()
                    .find(|u| &u.name == username && hash_match(password, u.password))
                    .cloned()
            };

            // If the login failed (this should be `None` but see above comment)
            if option_user.is_none() {
                let mut form = form::templates::log_in();
                form.set_defaults(vec![Some(username), None, None]);
                form.add_error("Username or password didn't match.");
                println!("event: failed login attempt: {} {}", username, password);
                let mut resp = Response::with(form.render(Box::new(get_user(r))));
                resp.headers.set(iron::headers::ContentType::html());
                return Ok(resp);
            };

            let user = option_user.unwrap().clone();

            // generate unique session ID
            let session_id = ProcessUniqueId::new();

            // put session id into cookie
            r.extensions
                .insert::<SessionKey>(Session { id: session_id });

            // insert mapping of session id to user object into store
            {
                get_store(r)
                    .write()
                    .unwrap()
                    .sessions
                    .insert(session_id, user.id);
            }
        }
        "register" => {
            let state = get_store(r);
            let username = params.get("username").unwrap();
            let password = params.get("password").unwrap();
            let email = params.get("email").unwrap();
            let password_confirm = params.get("pconfirm").unwrap();
            // This form is used to send the user back if there are any errors.
            let mut form = form::templates::register();
            form.set_defaults(vec![
                Some(username),
                Some(password),
                None,
                Some(email),
                None,
                None,
                None,
            ]);
            // Check if username or email is already registered
            if let Some(user) = state
                .read()
                .unwrap()
                .users
                .iter()
                .find(|u| &u.name == username || &u.email == email)
            {
                if &user.email == email {
                    form.add_error("Email is already registered.");
                }
                if &user.name == username {
                    form.add_error("Username is taken.")
                }
            }

            if username.len() < 3 {
                form.add_error("Username must be at least 3 letters long");
            }

            // Check if the password confirmation matched
            if password != password_confirm {
                form.add_error("Password confirmation did not match.");
            }

            // Check if the email is a valid email (just that it has a @ and a .)
            if !(email.contains('.') && email.contains('@')) {
                form.add_error("Invalid email provided")
            }

            if form.has_errors() {
                let mut resp = Response::with(form.render(Box::new(get_user(r))));
                resp.headers.set(iron::headers::ContentType::html());
                return Ok(resp);
            }

            // Hash the password
            let hashed_password = hash_password(password);

            // Add user to current vector of users
            let user_id = {
                state.write().unwrap().add_user(
                    username,
                    hashed_password,
                    UserRole::Normal,
                    Preferences {},
                )
            };
            // TODO abstract this login logic which is in log_in and register,
            // generate unique session ID
            let session_id = ProcessUniqueId::new();

            // put session id into cookie
            r.extensions
                .insert::<SessionKey>(Session { id: session_id });

            // insert mapping of session id to user object into store
            {
                state.write().unwrap().sessions.insert(session_id, user_id);
            }

            println!("event: new user registered: {} {}", username, email);
            // Send a welcome email to the user.
            let project_directory = env!("CARGO_MANIFEST_DIR");
            let mail_gun = Command::new(format!("{}/resources/mailgun.sh", project_directory))
                .arg(email)
                .output();

            println!("{:?}", mail_gun);
            return template(pages::welcome(Box::new(get_user(r))));
        }
        _ => (),
    }

    handlers::pages::home(r)

    // TODO go to user profile, or better yet the last page they were on
}
