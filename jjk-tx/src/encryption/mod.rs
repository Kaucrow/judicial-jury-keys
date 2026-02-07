pub mod encryption;
pub use encryption::Encrypter;

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct EncryptedPackage {
    encrypted_session_key_b64: String,
    encrypted_data_b64: String,
    nonce_b64: String,
    hash_b64: String,
}