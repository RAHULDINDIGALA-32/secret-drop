use aes_gcm::{
    Aes256Gcm,
    KeyInit,
    aead::{Aead, Nonce},
};
use anyhow::anyhow;
use rand::{RngExt, rng};

use crate::errors::SecretDropError;

pub type EncryptedPayload = (Vec<u8>, Vec<u8>, Vec<u8>);

pub fn encrypt(plaintext: &[u8]) -> Result<EncryptedPayload, SecretDropError> {
    let mut key = [0_u8; 32];
    let mut nonce = [0_u8; 12];

    let mut rng = rng();
    rng.fill(&mut key);
    rng.fill(&mut nonce);

    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|err| SecretDropError::InternalError(anyhow!("Inavlid AES Key: {err}")))?;

    let ciphertext = cipher
        .encrypt(&Nonce::<Aes256Gcm>::from(nonce), plaintext)
        .map_err(|err| SecretDropError::InternalError(anyhow!("Encryption failed: {err}")))?;

    Ok((ciphertext, nonce.to_vec(), key.to_vec()))
}

pub fn decrypt(ciphertext: &[u8], nonce: &[u8], key: &[u8]) -> Result<String, SecretDropError> {
    if key.len() != 32 || nonce.len() != 12 {
        return Err(SecretDropError::DecryptionFailed);
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|err| SecretDropError::InternalError(anyhow!("Inavlid AES Key: {err}")))?;

    let nonce = Nonce::<Aes256Gcm>::try_from(nonce)
        .map_err(|_| SecretDropError::DecryptionFailed)?;

    let plaintext = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| SecretDropError::DecryptionFailed)?;

    String::from_utf8(plaintext).map_err(|_| SecretDropError::DecryptionFailed)
}


#[cfg(test)]
mod tests {
    use super::{
        encrypt,
        decrypt
    };

    #[test]
    fn encrypt_decrypt_round_trip() {
        let input = b"Hello Team Alpha! Execute code RED_DRAGON at 0300 hours. Over.";
        let (ciphertext, nonce, key) = encrypt(input).expect("Encryption should work");
        let ouput = decrypt(&ciphertext, &nonce, &key).expect("Decrypt should work");

        assert_eq!(ouput, "Hello Team Alpha! Execute code RED_DRAGON at 0300 hours. Over.");
    }

    #[test]
    fn decrypt_fails_for_tampered_ciphertext() {
        let input = b"Hello Team Alpha! Execute code RED_DRAGON at 0300 hours. Over.";
        let (mut ciphertext, nonce, key) = encrypt(input).expect("Encryption should work");
        ciphertext[0] ^= 0xFF; // Tamper with the ciphertext

        let result = decrypt(&ciphertext, &nonce, &key);

        assert!(result.is_err());
    }
}
