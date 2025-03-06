//! Authentication utilities for the Kraken API

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{Error, Result};

/// Generate a nonce for API requests
pub fn generate_nonce() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Sign a message with the API secret
pub fn sign_message(path: &str, nonce: u64, postdata: &str, api_secret: &str) -> Result<String> {
    // Decode the API secret
    let secret = match BASE64.decode(api_secret) {
        Ok(s) => s,
        Err(_) => return Err(Error::Auth("Invalid API secret".to_string())),
    };

    // Create the message to sign
    // Format: nonce + postdata
    let message = format!("{}{}", nonce, postdata);

    // Create the SHA256 hash of the message
    let mut sha256 = Sha256::new();
    sha256.update(message.as_bytes());
    let message_hash = sha256.finalize();

    // Combine the path and the message hash
    let mut mac_data = Vec::new();
    mac_data.extend_from_slice(path.as_bytes());
    mac_data.extend_from_slice(&message_hash);

    // Create the HMAC-SHA512 signature
    let mut mac = Hmac::<Sha512>::new_from_slice(&secret)
        .map_err(|_| Error::Auth("Failed to create HMAC".to_string()))?;
    mac.update(&mac_data);
    let signature = mac.finalize().into_bytes();

    // Encode the signature in base64
    Ok(BASE64.encode(signature))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nonce() {
        let nonce1 = generate_nonce();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let nonce2 = generate_nonce();
        assert!(nonce2 > nonce1);
    }

    #[test]
    fn test_sign_message() {
        // This is just a basic test to ensure the function runs without errors
        // In a real test, you would use known inputs and expected outputs
        let path = "/0/private/Balance";
        let nonce = 1614232229325;
        let postdata = "nonce=1614232229325";
        let api_secret = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";
        
        let signature = sign_message(path, nonce, postdata, api_secret);
        assert!(signature.is_ok());
    }
}
