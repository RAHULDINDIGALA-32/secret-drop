use aes_gcm::KeyInit;
use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, generic_array::GenericArray},
};
use anyhow::anyhow;
use rand::RngCore;

use crate::errors::SecretDropError;

pub type EncryptedPayload = (Vec<u8>, Vec<u8>, Vec<u8>);

pub fn encrypt(plaintext: &[u8]) -> Result<EncryptedPayload, SecretDropError> {
    let mut key = vec![0_u8; 32];
    let mut nonce = vec![0_u8; 12];

    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut nonce);

    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|err| SecretDropError::InternalError(anyhow!("Inavlid AES Key: {err}")))?;

    let ciphertext = cipher
        .encrypt(GenericArray::from_slice(&nonce), plaintext)
        .map_err(|err| SecretDropError::InternalError(anyhow!("Encryption failed: {err}")))?;

    Ok((ciphertext, nonce, key))
}

pub fn decrypt(ciphertext: &[u8], nonce: &[u8], key: &[u8]) -> Result<String, SecretDropError> {
    if key.len() != 32 || nonce.len() != 12 {
        return Err(SecretDropError::DecryptionFailed);
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|err| SecretDropError::InternalError(anyhow!("Inavlid AES Key: {err}")))?;

    let plaintext = cipher
        .decrypt(GenericArray::from_slice(nonce), ciphertext)
        .map_err(|_| SecretDropError::DecryptionFailed)?;

    String::from_utf8(plaintext).map_err(|_| SecretDropError::DecryptionFailed)
}
