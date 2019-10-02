use iron::typemap;

use serde_derive::{Deserialize, Serialize};
use snowflake::ProcessUniqueId;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
}

pub struct SessionKey {}

impl typemap::Key for SessionKey {
    type Value = Session;
}

pub type SessionId = ProcessUniqueId;
