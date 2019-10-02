use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use crate::store::user::Password;

pub const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub const N_ITER: u32 = 300_000;
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

/// Hashes a password. Returns the hashed password and the salt as a tuple. Could maybe be put into its own "Security" module someday
/// if other security functions are ever needed.
pub fn hash_password(password: &str) -> Password {
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt).unwrap();

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        std::num::NonZeroU32::new(N_ITER).unwrap(),
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    return Password {
        hashed_password: pbkdf2_hash,
        salt,
    };
}

pub fn hash_match(password: &str, hashed_password: Password) -> bool {
    return pbkdf2::verify(
        PBKDF2_ALG,
        std::num::NonZeroU32::new(N_ITER).unwrap(),
        &hashed_password.salt,
        password.as_bytes(),
        &hashed_password.hashed_password,
    )
    .is_ok();
}
