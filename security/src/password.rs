use base64::{Engine, engine::general_purpose::STANDARD};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::{Rng, distributions::Alphanumeric};
use sha2::Sha256;

const ITERATIONS: u32 = 150000;
const SALT_LENGTH: usize = 12;
const KEY_LENGTH: usize = 32;

fn generate_salt() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SALT_LENGTH)
        .map(char::from)
        .collect()
}

pub fn hash_password(password: &str) -> String {
    hash_password_with_salt(password, &generate_salt())
}

pub fn hash_password_with_salt(password: &str, salt: &str) -> String {
    let mut key = vec![0u8; KEY_LENGTH];

    // pbkdf2 returns Result, but we can safely unwrap here as our parameters are valid
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt.as_bytes(), ITERATIONS, &mut key)
        .expect("PBKDF2 hashing failed");

    let hash = STANDARD.encode(&key);
    format!("pbkdf2_sha256${}${}${}", ITERATIONS, salt, hash)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    if !hash.starts_with("pbkdf2_sha256$") {
        return false;
    }

    let parts: Vec<&str> = hash.split('$').collect();
    if parts.len() != 4 {
        return false;
    }

    let iterations: u32 = parts[1].parse().unwrap_or(0);
    if iterations == 0 {
        return false;
    }

    let salt = parts[2];
    let expected_hash = parts[3];

    let mut key = vec![0u8; KEY_LENGTH];

    // Handle the Result properly
    if pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt.as_bytes(), iterations, &mut key).is_err() {
        return false;
    }

    let computed_hash = STANDARD.encode(&key);
    computed_hash == expected_hash
}

pub fn needs_rehash(hash: &str) -> bool {
    if !hash.starts_with("pbkdf2_sha256$") {
        return true;
    }

    let parts: Vec<&str> = hash.split('$').collect();
    if parts.len() != 4 {
        return true;
    }

    let iterations: u32 = parts[1].parse().unwrap_or(0);
    iterations != ITERATIONS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_format() {
        let password = "MySecurePassword123!";
        let hash = hash_password(password);

        // Check Django format
        assert!(hash.starts_with("pbkdf2_sha256$150000$"));
        let parts: Vec<&str> = hash.split('$').collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], "pbkdf2_sha256");
        assert_eq!(parts[1], "150000");
        assert_eq!(parts[2].len(), SALT_LENGTH);
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "MySecurePassword123!";
        let hash = hash_password(password);

        assert!(
            verify_password(password, &hash),
            "Password verification should succeed"
        );
    }

    #[test]
    fn test_verify_password_incorrect() {
        let password = "MySecurePassword123!";
        let wrong_password = "WrongPassword456!";
        let hash = hash_password(password);

        assert!(
            !verify_password(wrong_password, &hash),
            "Password verification should fail"
        );
    }

    #[test]
    fn test_hash_with_known_salt() {
        let password = "testpassword";
        let salt = "knownsalt123";
        let hash1 = hash_password_with_salt(password, salt);
        let hash2 = hash_password_with_salt(password, salt);

        // Same password and salt should produce same hash
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_salts() {
        let password = "MySecurePassword123!";
        let hash1 = hash_password(password);
        let hash2 = hash_password(password);

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_needs_rehash() {
        let current_hash = hash_password("password");
        assert!(!needs_rehash(&current_hash));

        // Old hash with different iterations
        let old_hash = "pbkdf2_sha256$100000$oldsalt$somehash";
        assert!(needs_rehash(old_hash));
    }

    #[test]
    fn test_invalid_hash_format() {
        assert!(!verify_password("password", "invalid_hash"));
        assert!(!verify_password(
            "password",
            "pbkdf2_sha256$notanumber$salt$hash"
        ));
    }

    #[test]
    fn test_unicode_password() {
        let password = "パスワード123!🔒";
        let hash = hash_password(password);

        assert!(verify_password(password, &hash));
    }
}
