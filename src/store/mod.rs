use bincode::{deserialize, serialize};

use iron::typemap::Key;
use iron::Plugin;
use iron::Request;
use persistent::State;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, RwLock};
use crate::utils::security::hash_password;

pub mod preferences;
pub mod session;
pub mod user;

use self::preferences::Preferences;
pub use self::session::{Session, SessionId, SessionKey};
pub use self::user::{Password, User, UserId, UserRole};

#[derive(Serialize, Deserialize, Clone)]
pub struct Store {
    /// All currently registered users on the site
    pub users: Vec<User>,
    /// All of the sessions that are in progress
    pub sessions: HashMap<SessionId, UserId>,
}

impl Store {
    pub fn save_to_storage(&self) -> Result<(), std::io::Error> {
        let encoded: Vec<u8> = serialize(self).expect("error serializing store");
        let mut pos = 0;
        let mut buffer = File::create("store.dat")?;
        while pos < encoded.len() {
            let bytes_written = buffer.write(&encoded[pos..])?;
            pos += bytes_written;
        }
        return Ok(());
    }

    /// Writes the store out to a json file. Not used for general data storage, but allows for easier inspection of the store
    /// and data. Also can help in case of recovery, if I need to manually migrate data or something.
    /// Hashmaps with non-primitive keys are unable to be serialized to json, so the sessions are emptied out before
    /// saving to json.
    pub fn save_to_json(&self) -> Result<(), std::io::Error> {
        let mut store = self.clone();
        store.sessions = HashMap::new();
        let encoded: String = serde_json::to_string(&store)
            .expect("unable to serialize to json")
            .chars()
            .map(|x| x.clone())
            .collect();
        let mut buffer = File::create("store.json")?;
        buffer.write_all(encoded.as_bytes())?;
        return Ok(());
    }

    /// Loads the store from a bincode-serialized file called "store.dat"
    pub fn load_from_storage() -> Result<Store, std::io::Error> {
        let mut f = File::open("store.dat")?;
        let mut buffer: Vec<u8> = Vec::new();

        f.read_to_end(&mut buffer)
            .expect("unable to read into buffer");

        let store = deserialize(&buffer[..]).expect("Failed to deserialize buffer");
        Ok(store)
    }

    /// Loads the store from a json file
    #[allow(dead_code)]
    pub fn load_from_json() -> Result<Store, std::io::Error> {
        let mut f = File::open("store.json")?;
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer)
            .expect("unable to read from file");

        let store = serde_json::from_slice(&buffer)?;
        Ok(store)
    }

    /// Gets a user out of the users vector by ID.
    pub fn get_user_by_id(&self, id: &UserId) -> Option<&User> {
        self.users.iter().find(|x| &x.id == id)
    }

    /// Adds a user to the store, auto-assigning an ID.
    pub fn add_user(
        &mut self,
        name: &str,
        pass: Password,
        role: UserRole,
        preferences: Preferences,
    ) -> UserId {
        let user_id: UserId = self.users.len();
        self.users
            .push(User::new(name, pass, role, &user_id, preferences));
        user_id
    }

    pub fn set_user_preferences(&mut self, user_id: UserId, preferences: Preferences) {
        let user = self
            .users
            .iter_mut()
            .find(|x| x.id == user_id)
            .expect("no user found with given id");
        user.set_preferences(preferences);
    }
}

/// A utility function which takes in a request and extracts the current user based on the session
/// within the request.
pub fn get_user(r: &mut Request) -> Option<User> {
    let store = get_store(r);
    if let Some(session) = r.extensions.get::<SessionKey>() {
        if let Some(id) = store.read().unwrap().sessions.get(&session.id) {
            return Some(
                store
                    .read()
                    .unwrap()
                    .users
                    .iter()
                    .find(|x| &x.id == id)
                    .expect("no user for session id")
                    .clone(),
            );
        }
    }
    None
}

/// A utility function to extract the current store out of the request.
pub fn get_store(r: &mut Request) -> Arc<RwLock<Store>> {
    r.get::<State<Store>>()
        .expect("failed to extract state from request")
}

/// Returns the in-memory shared persistent `State` middleware. Creates a data file if it doesn't already
/// exist. If it does, load from that data file.
pub fn configure_store() -> Result<State<Store>, ()> {
    if !Path::new("store.dat").exists() {
        Store {
            // TODO blank this out when testing is over
            users: vec![
                User::new(
                    "example_admin",
                    hash_password("example_password"), // good luck, hackers!
                    UserRole::Admin,
                    &0,
                    Preferences {},
                ),
                User::new(
                    "example_user",
                    hash_password("example_password"),
                    UserRole::Normal,
                    &1,
                    Preferences {},
                ),
            ],
            sessions: HashMap::<SessionId, UserId>::new(),
        }
        .save_to_storage()
        .unwrap();
    }
    let store = Store::load_from_storage().expect("Failed to load store.dat from storage");

    Ok(State::one(store))
}

impl Key for Store {
    type Value = Store;
}
