use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use password_hash::SaltString;
use rand::RngCore;
use std::env;
use std::fmt;
use std::fs::{self};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

// Custom error type to handle different error conversions
#[derive(Debug)]
enum EncryptionError {
    Argon2Error(argon2::Error),
    PasswordHashError(password_hash::Error),
    AesGcmError(aes_gcm::Error),
    Utf8Error(std::string::FromUtf8Error),
}

impl std::error::Error for EncryptionError {}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionError::Argon2Error(e) => write!(f, "Argon2 error: {}", e),
            EncryptionError::PasswordHashError(e) => write!(f, "Password hash error: {}", e),
            EncryptionError::AesGcmError(_) => write!(f, "AES-GCM encryption/decryption error"),
            EncryptionError::Utf8Error(e) => write!(f, "UTF-8 conversion error: {}", e),
        }
    }
}

impl From<argon2::Error> for EncryptionError {
    fn from(err: argon2::Error) -> Self {
        EncryptionError::Argon2Error(err)
    }
}

impl From<password_hash::Error> for EncryptionError {
    fn from(err: password_hash::Error) -> Self {
        EncryptionError::PasswordHashError(err)
    }
}

impl From<aes_gcm::Error> for EncryptionError {
    fn from(err: aes_gcm::Error) -> Self {
        EncryptionError::AesGcmError(err)
    }
}

impl From<std::string::FromUtf8Error> for EncryptionError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        EncryptionError::Utf8Error(err)
    }
}

// Function to derive a key from a password using Argon2
fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], EncryptionError> {
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(19456, 2, 1, Some(32)).unwrap(),
    );

    // Convert raw salt to SaltString
    let salt_string = SaltString::encode_b64(salt).map_err(EncryptionError::PasswordHashError)?;

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(EncryptionError::PasswordHashError)?;

    let hash = password_hash.hash.unwrap();
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash.as_bytes()[..32]);
    Ok(key)
}

/// Encrypt a secret string using the given key and nonce.
///
/// # Errors
///
/// This function will return an `EncryptionError` if the encryption fails.
///
/// # Examples
///
///
fn encrypt_secret(
    secret: &str,
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<Vec<u8>, EncryptionError> {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce);
    cipher
        .encrypt(nonce, secret.as_bytes())
        .map_err(EncryptionError::AesGcmError)
}

/// Decrypts a ciphertext using the provided key and nonce.
///
/// # Arguments
///
/// * `ciphertext` - A slice of bytes representing the encrypted data.
/// * `key` - A 32-byte array used as the key for decryption.
/// * `nonce` - A 12-byte array used as the nonce for decryption.
///
/// # Returns
///
/// Returns a `Result` containing the decrypted string if successful, or an `EncryptionError` if decryption fails.
///
/// # Errors
///
/// This function will return an `EncryptionError` if the decryption fails due to an invalid key, nonce, or if
/// the resulting plaintext cannot be converted to a UTF-8 string.
fn decrypt_secret(
    ciphertext: &[u8],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<String, EncryptionError> {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(EncryptionError::AesGcmError)?;
    String::from_utf8(plaintext).map_err(EncryptionError::Utf8Error)
}

/// A test function that demonstrates the encryption and decryption of a secret message using a derived key from a password.
///
/// The test generates a random salt and nonce, derives a key from the given password, encrypts a secret message using the key and nonce,
/// decrypts the ciphertext using the same key and nonce, and verifies that the decrypted message matches the original.
///
/// Additionally, the test demonstrates that decrypting the ciphertext with a wrong password will fail.
///
/// # Errors
///
/// Returns an error if the encryption, decryption, or key derivation fails.
pub fn mock_encrypt_decrypt_test() -> Result<(), Box<dyn std::error::Error>> {
    let password = "super_secure_password123";
    let secret = "This is a top secret message!";

    // Generate random salt and nonce
    let mut rng = rand::rng();
    let mut salt = [0u8; 16];
    let mut nonce = [0u8; 12];
    rng.fill_bytes(&mut salt);
    rng.fill_bytes(&mut nonce);

    // Derive key from password
    let key = derive_key(password, &salt).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Encrypt the secret
    let ciphertext = encrypt_secret(secret, &key, &nonce)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("Encrypted ciphertext: {:?}", ciphertext);

    // Decrypt the secret
    let decrypted = decrypt_secret(&ciphertext, &key, &nonce)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("Decrypted secret: {}", decrypted);

    // Verify the decryption
    assert_eq!(
        secret, decrypted,
        "Decrypted secret does not match original!"
    );

    // Test wrong password
    let wrong_password = "wrong_password";
    let wrong_key =
        derive_key(wrong_password, &salt).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let decrypt_result = decrypt_secret(&ciphertext, &wrong_key, &nonce);
    assert!(
        decrypt_result.is_err(),
        "Decryption with wrong password should fail!"
    );

    Ok(())
}

/// Checks if a file exists at the given path and has user read-only permissions (u+r).
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file.
///
/// # Returns
///
/// * `Ok(true)` if the file exists and has user read-only permissions (0400).
/// * `Ok(false)` if the file does not exist, is not a file, or does not have the required permissions.
/// * `Err` if there is an error accessing the file's metadata.
pub fn check_password_file(file_path: &str) -> Result<bool, std::io::Error> {
    let path = Path::new(file_path);

    // Check if the file exists
    if !path.exists() || !path.is_file() {
        return Ok(false);
    }

    // Get the file's metadata
    let metadata = fs::metadata(path)?;

    // Get the file's permissions
    let permissions = metadata.permissions();

    // Check if permissions are exactly u+r (0400 in octal)
    let mode = permissions.mode();
    let user_read_only = 0o400;
    Ok(mode & 0o777 == user_read_only)
}

fn main() {
    let password_file = ".password.secret";
    mock_encrypt_decrypt_test().unwrap();
    let exists = check_password_file(password_file).unwrap();
    if exists {
        println!("File {} exists and has u+r permissions", password_file);
    } else {
        println!("File does not exist or does not have u+r permissions");
        println!(
            "Current directory: {}",
            env::current_dir().unwrap().display()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, Permissions, set_permissions};
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_mock_encrypt_decrypt() {
        assert!(mock_encrypt_decrypt_test().is_ok());
    }

    #[test]
    fn test_check_password_file() -> Result<(), std::io::Error> {
        // Create a temporary file
        let file_path = "test_file.txt";
        File::create(file_path)?;

        // Set permissions to u+r (0400)
        let permissions = Permissions::from_mode(0o400);
        set_permissions(file_path, permissions.clone())?;

        // Test the function
        assert_eq!(
            check_password_file(file_path)?,
            true,
            "File should have u+r permissions"
        );

        // Change permissions to something else (e.g., u+rw)
        let wrong_permissions = Permissions::from_mode(0o600);
        set_permissions(file_path, wrong_permissions)?;

        // Test with incorrect permissions
        assert_eq!(
            check_password_file(file_path)?,
            false,
            "File should not have u+r permissions"
        );

        // Clean up
        fs::remove_file(file_path)?;

        // Test non-existent file
        assert_eq!(
            check_password_file("non_existent.txt")?,
            false,
            "Non-existent file should return false"
        );

        Ok(())
    }
}
