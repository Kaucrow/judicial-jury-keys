use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RxPayload {
    pub pdf_id: String,
    pub pkg: EncryptedPackage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedPackage {
    pub encrypted_session_key_b64: String,
    pub encrypted_data_b64: String,
    pub nonce_b64: String,
    pub hash_b64: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RxKeyResponse {
    pub pdf_id: String,
    pub pub_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PdfData {
    pub title: String,
    pub subject: String,
    pub author: String,
    pub keywords: String,
    pub file: Vec<u8>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CaseSummary {
    pub case_code: String,
    pub file_path: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
