pub mod transmitter;

pub use transmitter::Transmitter;

use crate::prelude::*;
use crate::encryption::EncryptedPackage;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RxKeyResponse {
    pdf_id: String,
    pub_key: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RxPayload {
    pdf_id: String,
    pkg: EncryptedPackage,
}