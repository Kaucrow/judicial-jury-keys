use crate::prelude::*;
use crate::transmission::Transmitter;
use super::EncryptedPackage;

pub struct Encrypter {}

impl Encrypter {
    pub async fn perform_hybrid_encryption(
        msg_bytes: &[u8],
    ) -> anyhow::Result<EncryptedPackage> {
        // Hash the message bytes
        let mut hasher = Sha256::new();
        hasher.update(msg_bytes);
        let hash = hasher.finalize();

        // Fetch public key from RX
        let rx_pub_key = Transmitter::get_pub_key().await?;

        let mut rng = OsRng;

        // Generate AES session key
        let session_key = Aes256Gcm::generate_key(&mut rng);
        debug!("Generated AES session key");

        // Encrypt message (PDF bytes)
        let cipher = Aes256Gcm::new(&session_key);
        let nonce = Aes256Gcm::generate_nonce(&mut rng);
        let encrypted_data = cipher.encrypt(&nonce, msg_bytes)
            .map_err(|e| anyhow!("AES error: {}", e))?;
        debug!("Encrypted message with AES session key");

        // Encrypt session key with RSA
        let encrypted_session_key = rx_pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, session_key.as_slice())
            .map_err(|e| anyhow!("RSA error: {}", e))?;
        debug!("Encrypted AES session key with RX public key");

        Ok(EncryptedPackage {
            encrypted_session_key_b64: b64.encode(encrypted_session_key),
            encrypted_data_b64: b64.encode(encrypted_data),
            nonce_b64: b64.encode(nonce),
            hash_b64: b64.encode(hash),
        })
    }
}