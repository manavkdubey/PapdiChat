use crate::error::Result;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use magic_crypt::{MagicCryptTrait, new_magic_crypt};

pub fn enc(secret: &[u8], pass: &str) -> String {
    let mc = new_magic_crypt!(pass, 256);
    mc.encrypt_bytes_to_base64(secret)
}

pub fn dec(ct: &str, pass: &str) -> Result<Vec<u8>> {
    let mc = new_magic_crypt!(pass, 256);
    Ok(mc.decrypt_base64_to_bytes(ct)?)
}

pub fn hash_password(pwd: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(pwd.as_bytes(), &salt).unwrap();
    hash.to_string()
}

pub fn verify_password(hash_str: &str, pwd: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash_str).unwrap();
    Argon2::default()
        .verify_password(pwd.as_bytes(), &parsed_hash)
        .is_ok()
}
