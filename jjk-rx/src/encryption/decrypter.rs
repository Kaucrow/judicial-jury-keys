use crate::prelude::*;

pub struct Decrypter;

impl Decrypter {
    pub fn generate_keys() -> Result<(RsaPrivateKey, String)> {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let public_key = RsaPublicKey::from(&private_key);
        let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;
        Ok((private_key, public_key_pem))
    }

    pub fn decrypt_hybrid(
        private_key: &RsaPrivateKey,
        encrypted_session_key_b64: &str,
        encrypted_data_b64: &str,
        nonce_b64: &str,
    ) -> Result<Vec<u8>> {
        let enc_session_key = b64.decode(encrypted_session_key_b64)
            .map_err(|e| anyhow!("Failed to decode session key: {}", e))?;
        let enc_data = b64.decode(encrypted_data_b64)
            .map_err(|e| anyhow!("Failed to decode encrypted data: {}", e))?;
        let nonce_bytes = b64.decode(nonce_b64)
            .map_err(|e| anyhow!("Failed to decode nonce: {}", e))?;
        let session_key = private_key.decrypt(Pkcs1v15Encrypt, &enc_session_key)
            .map_err(|e| anyhow!("RSA Decryption failed: {}", e))?;

        let cipher = Aes256Gcm::new_from_slice(&session_key)
            .map_err(|e| anyhow!("Invalid AES key length: {}", e))?;
        
        if nonce_bytes.len() != 12 {
            return Err(anyhow!("Invalid nonce length"));
        }
        let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);
        
        let plaintext = cipher.decrypt(nonce, enc_data.as_ref())
            .map_err(|e| anyhow!("AES Decryption failed: {}", e))?;

        Ok(plaintext)
    }

    pub fn verify_hash(data: &[u8], hash_b64: &str) -> Result<bool> {
        let expected_hash = b64.decode(hash_b64)
            .map_err(|e| anyhow!("Failed to decode hash: {}", e))?;
            
        let mut hasher = Sha256::new();
        hasher.update(data);
        let actual_hash = hasher.finalize();
        
        Ok(actual_hash.as_slice() == expected_hash.as_slice())
    }
}
