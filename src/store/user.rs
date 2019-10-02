use chrono::{DateTime, Utc};
pub type UserId = usize;
use super::preferences::Preferences;
use serde_derive::{Deserialize, Serialize};

use crate::utils::security::CREDENTIAL_LEN;
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub id: UserId,
    pub email: String,
    pub password: Password,
    premium_until: DateTime<Utc>,
    created: DateTime<Utc>,
    confirmed: bool,
    pub role: UserRole,
    pub preferences: Preferences,
}

impl User {
    pub fn new(
        name: &str,
        pass: Password,
        role: UserRole,
        user_id: &UserId,
        preferences: Preferences,
    ) -> User {
        User {
            id: user_id.clone(),
            name: name.to_owned(),
            email: String::new(),
            password: pass,
            premium_until: Utc::now(),
            created: Utc::now(),
            confirmed: false,
            role,
            preferences,
        }
    }

    pub fn set_preferences(&mut self, preferences: Preferences) {
        self.preferences = preferences;
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Normal,
    Moderator,
    Admin,
}
impl UserRole {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            UserRole::Normal => "Normal",
            UserRole::Moderator => "Moderator",
            UserRole::Admin => "Admin",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Password {
    pub hashed_password: [u8; CREDENTIAL_LEN],
    pub salt: [u8; CREDENTIAL_LEN],
}
