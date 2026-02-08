use crate::prelude::*;
use crate::{
    settings::get_settings,
    encryption::EncryptedPackage,
};
use super::{RxKeyResponse, RxPayload};

pub struct Transmitter {}

impl Transmitter {
    pub async fn get_pub_key() -> anyhow::Result<(String, RsaPublicKey)>{
        let settings = get_settings()?;

        // Fetch public key from RX
        let rx_url = format!("http://{}:{}/{}", settings.rx.host, settings.rx.port, settings.rx.pub_key_endp);
        debug!("Fetching public key from RX...");

        let client = reqwest::Client::new();

        let response = client.get(&rx_url)
            .send()
            .await?
            .json::<RxKeyResponse>()
            .await?;

        debug!("Got RX public key for PDF ID '{}'", response.pdf_id);

        // Parse the PEM string from the JSON field
        let pub_key = RsaPublicKey::from_public_key_pem(&response.pub_key)
            .or_else(|_| RsaPublicKey::from_pkcs1_pem(&response.pub_key))?;

        // Return tuple (PDF ID, RsaPublicKey)
        Ok((response.pdf_id, pub_key))
    }

    pub async fn send_encrypted_pkg(pdf_id: String, pkg: EncryptedPackage) -> anyhow::Result<HttpResponse> {
        let client = reqwest::Client::new();
        let settings = get_settings()?;

        let rx_url = format!("http://{}:{}/{}", settings.rx.host, settings.rx.port, settings.rx.rcv_endp);
        debug!("Sending payload to RX for PDF ID '{}'", pdf_id);

        // Send PDF ID and Encrypted Package to RX
        let payload = RxPayload { pdf_id: pdf_id.clone(), pkg };

        let rx_response = client.post(&rx_url)
            .json(&payload)
            .send()
            .await?;

        let raw_status = rx_response.status().as_u16();
        let status = actix_web::http::StatusCode::from_u16(raw_status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        let body = rx_response.text().await.unwrap_or_else(|_| "Could not read RX response".to_string());

        if status.is_success() {
            debug!("Payload sent to RX for PDF ID '{}'", pdf_id);
        }

        Ok(HttpResponse::build(status).body(body))
    }
}