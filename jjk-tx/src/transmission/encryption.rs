use crate::prelude::*;
use crate::settings::{get_settings, Settings};

#[derive(Serialize, Deserialize)]
pub struct EncryptedPackage {
    encrypted_session_key_b64: String,
    encrypted_data_b64: String,
    nonce_b64: String,
    hash_b64: String,
}

pub async fn perform_hybrid_encryption(
    msg_bytes: &[u8], 
) -> anyhow::Result<EncryptedPackage> {
    let settings: Settings = get_settings()?;

    // Hash the message bytes
    let mut hasher = Sha256::new();
    hasher.update(msg_bytes);
    let hash = hasher.finalize();

    // Fetch public key from RX
    let rx_url = format!("http://{}:{}/{}", settings.rx.host, settings.rx.port, settings.rx.pub_key_endp);
    debug!("Fetching public key from RX...");

    let client = reqwest::Client::new();

    // RX server should return a raw PEM string in the body
    let pem_string = client.get(rx_url)
        .send()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, anyhow!("Reqwest error: {}", e)))?
        .text()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, anyhow!("Text error: {}", e)))?;

    debug!("Got RX public key {}", pem_string);

    // Parse the PEM into an RsaPublicKey
    // Use from_public_key_pem (PKCS8) or from_pkcs1_pem depending on format.
    // PKCS8 is the modern standard.
    let rx_pub_key = RsaPublicKey::from_public_key_pem(&pem_string)
        .or_else(|_| RsaPublicKey::from_pkcs1_pem(&pem_string)) // Fallback if format differs
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("PEM Parse error: {}", e)))?;

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