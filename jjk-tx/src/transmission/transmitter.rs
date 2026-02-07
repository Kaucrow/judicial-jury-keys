use crate::prelude::*;
use crate::{
    settings::get_settings,
    encryption::EncryptedPackage,
};

pub struct Transmitter {}

impl Transmitter {
    pub async fn get_pub_key() -> anyhow::Result<RsaPublicKey> {
        let settings = get_settings()?;

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
        Ok(RsaPublicKey::from_public_key_pem(&pem_string)
            .or_else(|_| RsaPublicKey::from_pkcs1_pem(&pem_string)) // Fallback if format differs
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("PEM Parse error: {}", e)))?)
    }

    pub async fn send_encrypted_pkg(pkg: &EncryptedPackage) -> anyhow::Result<HttpResponse> {
        let client = reqwest::Client::new();

        let settings = get_settings()?;

        debug!("Sending encrypted package to RX...");

        let rx_response = client.post(
            format!(
                "http://{}:{}/{}", settings.rx.host, settings.rx.port, settings.rx.rcv_endp)
            )
            .json(pkg)
            .send()
            .await?;
        
        let raw_status = rx_response.status().as_u16();
        let status = actix_web::http::StatusCode::from_u16(raw_status)
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        let body = rx_response.text().await.unwrap_or_else(|_| "Could not read RX response".to_string());

        debug!("Encrypted package sent");

        Ok(HttpResponse::build(status).body(body))
    }
}